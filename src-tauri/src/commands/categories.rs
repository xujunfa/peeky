use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub sort_order: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategoryInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCategoryInput {
    pub id: i64,
    pub name: Option<String>,
    pub sort_order: Option<i64>,
}

// --- Pool-based functions (testable) ---

pub async fn get_categories_by_pool(db: &SqlitePool) -> Result<Vec<Category>, String> {
    sqlx::query_as::<_, Category>(
        "SELECT id, name, sort_order, created_at, updated_at FROM categories ORDER BY sort_order, id",
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.to_string())
}

pub async fn create_category_by_pool(
    db: &SqlitePool,
    input: CreateCategoryInput,
) -> Result<Category, String> {
    let max_order: Option<(i64,)> =
        sqlx::query_as("SELECT COALESCE(MAX(sort_order), -1) FROM categories")
            .fetch_optional(db)
            .await
            .map_err(|e| e.to_string())?;
    let next_order = max_order.map(|r| r.0 + 1).unwrap_or(0);

    let id = sqlx::query("INSERT INTO categories (name, sort_order) VALUES (?, ?)")
        .bind(&input.name)
        .bind(next_order)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?
        .last_insert_rowid();

    sqlx::query_as::<_, Category>(
        "SELECT id, name, sort_order, created_at, updated_at FROM categories WHERE id = ?",
    )
    .bind(id)
    .fetch_one(db)
    .await
    .map_err(|e| e.to_string())
}

pub async fn update_category_by_pool(
    db: &SqlitePool,
    input: UpdateCategoryInput,
) -> Result<Category, String> {
    let current = sqlx::query_as::<_, Category>(
        "SELECT id, name, sort_order, created_at, updated_at FROM categories WHERE id = ?",
    )
    .bind(input.id)
    .fetch_optional(db)
    .await
    .map_err(|e| e.to_string())?
    .ok_or_else(|| format!("Category {} not found", input.id))?;

    let name = input.name.unwrap_or(current.name);
    let sort_order = input.sort_order.unwrap_or(current.sort_order);

    sqlx::query(
        "UPDATE categories SET name = ?, sort_order = ?, updated_at = datetime('now') WHERE id = ?",
    )
    .bind(&name)
    .bind(sort_order)
    .bind(input.id)
    .execute(db)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query_as::<_, Category>(
        "SELECT id, name, sort_order, created_at, updated_at FROM categories WHERE id = ?",
    )
    .bind(input.id)
    .fetch_one(db)
    .await
    .map_err(|e| e.to_string())
}

pub async fn delete_category_by_pool(db: &SqlitePool, id: i64) -> Result<(), String> {
    let result = sqlx::query("DELETE FROM categories WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    if result.rows_affected() == 0 {
        return Err(format!("Category {} not found", id));
    }
    Ok(())
}

pub async fn reorder_categories_by_pool(db: &SqlitePool, ids: Vec<i64>) -> Result<(), String> {
    for (i, id) in ids.iter().enumerate() {
        sqlx::query(
            "UPDATE categories SET sort_order = ?, updated_at = datetime('now') WHERE id = ?",
        )
        .bind(i as i64)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

// --- Tauri commands (thin wrappers) ---

#[tauri::command]
pub async fn get_categories(db: State<'_, SqlitePool>) -> Result<Vec<Category>, String> {
    get_categories_by_pool(db.inner()).await
}

#[tauri::command]
pub async fn create_category(
    db: State<'_, SqlitePool>,
    input: CreateCategoryInput,
) -> Result<Category, String> {
    create_category_by_pool(db.inner(), input).await
}

#[tauri::command]
pub async fn update_category(
    db: State<'_, SqlitePool>,
    input: UpdateCategoryInput,
) -> Result<Category, String> {
    update_category_by_pool(db.inner(), input).await
}

#[tauri::command]
pub async fn delete_category(db: State<'_, SqlitePool>, id: i64) -> Result<(), String> {
    delete_category_by_pool(db.inner(), id).await
}

#[tauri::command]
pub async fn reorder_categories(db: State<'_, SqlitePool>, ids: Vec<i64>) -> Result<(), String> {
    reorder_categories_by_pool(db.inner(), ids).await
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;

    async fn setup_db() -> SqlitePool {
        let db = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("connect sqlite memory");
        sqlx::query(include_str!("../../migrations/001_init.sql"))
            .execute(&db)
            .await
            .expect("run migration 001");
        // 002 has two statements; execute them separately
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS categories (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              name TEXT NOT NULL,
              sort_order INTEGER NOT NULL DEFAULT 0,
              created_at TEXT NOT NULL DEFAULT (datetime('now')),
              updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
        )
        .execute(&db)
        .await
        .expect("create categories");
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS items (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
              label TEXT NOT NULL,
              value TEXT NOT NULL DEFAULT '',
              sort_order INTEGER NOT NULL DEFAULT 0,
              created_at TEXT NOT NULL DEFAULT (datetime('now')),
              updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
        )
        .execute(&db)
        .await
        .expect("create items");
        // Enable foreign keys
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&db)
            .await
            .expect("enable foreign keys");
        db
    }

    #[tokio::test]
    async fn get_categories_returns_empty_initially() {
        let db = setup_db().await;
        let cats = get_categories_by_pool(&db).await.unwrap();
        assert!(cats.is_empty());
    }

    #[tokio::test]
    async fn create_category_returns_new_category() {
        let db = setup_db().await;
        let cat = create_category_by_pool(
            &db,
            CreateCategoryInput {
                name: "Shortcuts".to_string(),
            },
        )
        .await
        .unwrap();
        assert_eq!(cat.name, "Shortcuts");
        assert_eq!(cat.sort_order, 0);
    }

    #[tokio::test]
    async fn create_category_auto_increments_sort_order() {
        let db = setup_db().await;
        let c1 = create_category_by_pool(
            &db,
            CreateCategoryInput {
                name: "A".to_string(),
            },
        )
        .await
        .unwrap();
        let c2 = create_category_by_pool(
            &db,
            CreateCategoryInput {
                name: "B".to_string(),
            },
        )
        .await
        .unwrap();
        assert_eq!(c1.sort_order, 0);
        assert_eq!(c2.sort_order, 1);
    }

    #[tokio::test]
    async fn update_category_partial_fields() {
        let db = setup_db().await;
        let cat = create_category_by_pool(
            &db,
            CreateCategoryInput {
                name: "Old".to_string(),
            },
        )
        .await
        .unwrap();

        let updated = update_category_by_pool(
            &db,
            UpdateCategoryInput {
                id: cat.id,
                name: Some("New".to_string()),
                sort_order: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(updated.name, "New");
        assert_eq!(updated.sort_order, cat.sort_order);
    }

    #[tokio::test]
    async fn delete_category_removes_it() {
        let db = setup_db().await;
        let cat = create_category_by_pool(
            &db,
            CreateCategoryInput {
                name: "ToDelete".to_string(),
            },
        )
        .await
        .unwrap();

        delete_category_by_pool(&db, cat.id).await.unwrap();

        let cats = get_categories_by_pool(&db).await.unwrap();
        assert!(cats.is_empty());
    }

    #[tokio::test]
    async fn delete_category_not_found() {
        let db = setup_db().await;
        let result = delete_category_by_pool(&db, 999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn reorder_categories_updates_sort_order() {
        let db = setup_db().await;
        let c1 = create_category_by_pool(
            &db,
            CreateCategoryInput {
                name: "A".to_string(),
            },
        )
        .await
        .unwrap();
        let c2 = create_category_by_pool(
            &db,
            CreateCategoryInput {
                name: "B".to_string(),
            },
        )
        .await
        .unwrap();

        // Reverse order
        reorder_categories_by_pool(&db, vec![c2.id, c1.id])
            .await
            .unwrap();

        let cats = get_categories_by_pool(&db).await.unwrap();
        assert_eq!(cats[0].name, "B");
        assert_eq!(cats[1].name, "A");
    }
}
