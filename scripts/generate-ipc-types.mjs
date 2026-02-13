import fs from "node:fs";
import path from "node:path";

const ROOT = process.cwd();
const TAURI_LIB = path.join(ROOT, "src-tauri/src/lib.rs");
const COMMANDS_DIR = path.join(ROOT, "src-tauri/src/commands");
const TYPES_FILE = path.join(ROOT, "src/types/index.ts");
const OUTPUT_FILE = path.join(ROOT, "src/core/ipc.generated.ts");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

function listRustFiles() {
  const commandFiles = fs
    .readdirSync(COMMANDS_DIR)
    .filter((name) => name.endsWith(".rs"))
    .map((name) => path.join(COMMANDS_DIR, name));

  return [TAURI_LIB, ...commandFiles];
}

function splitTopLevel(input, delimiter = ",") {
  const result = [];
  let current = "";
  let depthAngle = 0;
  let depthParen = 0;
  let depthBracket = 0;
  let depthBrace = 0;

  for (const char of input) {
    if (char === "<") depthAngle += 1;
    if (char === ">") depthAngle = Math.max(0, depthAngle - 1);
    if (char === "(") depthParen += 1;
    if (char === ")") depthParen = Math.max(0, depthParen - 1);
    if (char === "[") depthBracket += 1;
    if (char === "]") depthBracket = Math.max(0, depthBracket - 1);
    if (char === "{") depthBrace += 1;
    if (char === "}") depthBrace = Math.max(0, depthBrace - 1);

    if (
      char === delimiter &&
      depthAngle === 0 &&
      depthParen === 0 &&
      depthBracket === 0 &&
      depthBrace === 0
    ) {
      result.push(current.trim());
      current = "";
      continue;
    }

    current += char;
  }

  if (current.trim()) {
    result.push(current.trim());
  }

  return result;
}

function snakeToCamel(name) {
  return name.replace(/_([a-z])/g, (_, c) => c.toUpperCase());
}

function extractKnownTypes() {
  const source = read(TYPES_FILE);
  const types = new Set();
  const regex = /export interface\s+(\w+)/g;
  let match;

  while ((match = regex.exec(source)) !== null) {
    types.add(match[1]);
  }

  return types;
}

function extractRegisteredCommands() {
  const source = read(TAURI_LIB);
  const match = source.match(/generate_handler!\[([\s\S]*?)\]\)/);
  if (!match) {
    throw new Error("Could not find tauri::generate_handler![] in src-tauri/src/lib.rs");
  }

  const block = match[1]
    .split("\n")
    .map((line) => line.replace(/\/\/.*$/, "").trim())
    .filter(Boolean)
    .join(" ");

  return splitTopLevel(block)
    .map((item) => item.replace(/,$/, "").trim())
    .filter(Boolean)
    .map((item) => {
      const parts = item.split("::");
      return parts[parts.length - 1];
    });
}

function extractStructs(rustSource) {
  const structs = new Map();
  const structRegex = /pub struct\s+(\w+)\s*\{([\s\S]*?)\}/g;
  let match;

  while ((match = structRegex.exec(rustSource)) !== null) {
    const structName = match[1];
    const body = match[2];
    const fields = [];

    for (const line of body.split("\n")) {
      const field = line.trim().match(/^pub\s+(\w+)\s*:\s*([^,]+),?$/);
      if (field) {
        fields.push({ name: field[1], rustType: field[2].trim() });
      }
    }

    structs.set(structName, fields);
  }

  return structs;
}

function extractCommands(rustSource) {
  const commands = new Map();
  const commandRegex = /#\[tauri::command\]\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*([^\{]+))?\s*\{/g;
  let match;

  while ((match = commandRegex.exec(rustSource)) !== null) {
    const name = match[1];
    const paramsBlock = match[2].trim();
    const returnTypeRaw = (match[3] || "").trim();

    const params = splitTopLevel(paramsBlock)
      .map((item) => item.trim())
      .filter(Boolean)
      .map((item) => {
        const idx = item.indexOf(":");
        if (idx < 0) {
          return null;
        }

        return {
          name: item.slice(0, idx).trim(),
          rustType: item.slice(idx + 1).trim(),
        };
      })
      .filter((item) => item !== null);

    commands.set(name, {
      name,
      params,
      returnTypeRaw,
    });
  }

  return commands;
}

function unwrapResult(rustType) {
  if (!rustType.startsWith("Result<")) {
    return rustType;
  }

  const inner = rustType.slice("Result<".length, -1);
  const parts = splitTopLevel(inner);
  return parts[0].trim();
}

function rustTypeToTs(rustType, knownTypes, generatedStructs) {
  const t = rustType.replace(/\s+/g, " ").trim();

  if (t === "()") return "void";
  if (t === "String" || t === "&str") return "string";
  if (["i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64"].includes(t)) return "number";
  if (t === "bool") return "boolean";

  if (t.startsWith("Option<") && t.endsWith(">")) {
    const inner = t.slice("Option<".length, -1);
    return `${rustTypeToTs(inner, knownTypes, generatedStructs)} | null`;
  }

  if (t.startsWith("Vec<") && t.endsWith(">")) {
    const inner = t.slice("Vec<".length, -1);
    return `${rustTypeToTs(inner, knownTypes, generatedStructs)}[]`;
  }

  if (knownTypes.has(t) || generatedStructs.has(t)) {
    return t;
  }

  return "unknown";
}

function buildGeneratedFile({
  commands,
  registeredNames,
  structs,
  knownTypes,
}) {
  const generatedStructs = new Set(structs.keys());
  const commandEntries = [];
  const usedKnownTypes = new Set();
  const inlineStructs = new Set();

  for (const commandName of registeredNames) {
    const command = commands.get(commandName);
    if (!command) {
      throw new Error(`Command '${commandName}' is registered but no #[tauri::command] function was found.`);
    }

    const argFields = [];
    for (const param of command.params) {
      if (
        param.name === "db" ||
        param.rustType.includes("State<") ||
        param.rustType.includes("AppHandle")
      ) {
        continue;
      }

      const fieldName = snakeToCamel(param.name);
      const tsType = rustTypeToTs(param.rustType, knownTypes, generatedStructs);
      argFields.push({ name: fieldName, tsType });

      if (knownTypes.has(tsType)) usedKnownTypes.add(tsType);
      if (generatedStructs.has(tsType) && !knownTypes.has(tsType)) {
        inlineStructs.add(tsType);
      }
    }

    const unwrappedReturn = unwrapResult(command.returnTypeRaw || "()");
    const returnType = rustTypeToTs(unwrappedReturn, knownTypes, generatedStructs);

    if (knownTypes.has(returnType)) usedKnownTypes.add(returnType);
    if (generatedStructs.has(returnType) && !knownTypes.has(returnType)) {
      inlineStructs.add(returnType);
    }

    if (returnType.endsWith("[]")) {
      const inner = returnType.slice(0, -2);
      if (knownTypes.has(inner)) usedKnownTypes.add(inner);
      if (generatedStructs.has(inner) && !knownTypes.has(inner)) {
        inlineStructs.add(inner);
      }
    }

    commandEntries.push({
      name: commandName,
      args: argFields,
      returnType,
    });
  }

  const lines = [];
  lines.push("/**");
  lines.push(" * AUTO-GENERATED FILE. DO NOT EDIT.");
  lines.push(" * Generated by scripts/generate-ipc-types.mjs");
  lines.push(" */");
  lines.push("");

  const imports = [...usedKnownTypes].sort();
  if (imports.length > 0) {
    lines.push(`import type { ${imports.join(", ")} } from \"@/types\";`);
    lines.push("");
  }

  for (const structName of [...inlineStructs].sort()) {
    const fields = structs.get(structName) || [];
    lines.push(`export interface ${structName} {`);
    for (const field of fields) {
      const fieldType = rustTypeToTs(field.rustType, knownTypes, generatedStructs);
      lines.push(`  ${field.name}: ${fieldType};`);
    }
    lines.push("}");
    lines.push("");
  }

  lines.push("export const COMMAND_NAMES = [");
  for (const entry of commandEntries) {
    lines.push(`  \"${entry.name}\",`);
  }
  lines.push("] as const;");
  lines.push("");

  lines.push("export interface CommandArgs {");
  for (const entry of commandEntries) {
    if (entry.args.length === 0) {
      lines.push(`  ${entry.name}: Record<string, never>;`);
      continue;
    }

    const fields = entry.args.map((arg) => `${arg.name}: ${arg.tsType}`).join("; ");
    lines.push(`  ${entry.name}: { ${fields} };`);
  }
  lines.push("}");
  lines.push("");

  lines.push("export interface CommandReturns {");
  for (const entry of commandEntries) {
    lines.push(`  ${entry.name}: ${entry.returnType};`);
  }
  lines.push("}");
  lines.push("");

  return `${lines.join("\n")}\n`;
}

function main() {
  const checkOnly = process.argv.includes("--check");
  const rustFiles = listRustFiles();
  const knownTypes = extractKnownTypes();
  const registeredNames = extractRegisteredCommands();

  const mergedStructs = new Map();
  const mergedCommands = new Map();

  for (const file of rustFiles) {
    const source = read(file);
    const structs = extractStructs(source);
    const commands = extractCommands(source);

    for (const [name, fields] of structs.entries()) {
      mergedStructs.set(name, fields);
    }

    for (const [name, command] of commands.entries()) {
      mergedCommands.set(name, command);
    }
  }

  const output = buildGeneratedFile({
    commands: mergedCommands,
    registeredNames,
    structs: mergedStructs,
    knownTypes,
  });

  if (checkOnly) {
    const current = fs.existsSync(OUTPUT_FILE) ? fs.readFileSync(OUTPUT_FILE, "utf8") : "";
    if (current !== output) {
      // eslint-disable-next-line no-console
      console.error(
        `IPC types are out of date. Run: pnpm gen:ipc\nExpected file: ${path.relative(ROOT, OUTPUT_FILE)}`
      );
      process.exit(1);
    }
    // eslint-disable-next-line no-console
    console.log(`IPC types are up to date: ${path.relative(ROOT, OUTPUT_FILE)}`);
    return;
  }

  fs.writeFileSync(OUTPUT_FILE, output, "utf8");
  // eslint-disable-next-line no-console
  console.log(`Generated ${path.relative(ROOT, OUTPUT_FILE)}`);
}

main();
