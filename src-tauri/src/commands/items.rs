use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Item {
    pub id: i64,
    pub category_id: i64,
    pub label: String,
    pub value: String,
    pub sort_order: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// Flat struct returned by get_all_items (JOIN with categories).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ItemWithCategory {
    pub id: i64,
    pub category_id: i64,
    pub label: String,
    pub value: String,
    pub sort_order: i64,
    pub category_name: String,
    pub category_sort_order: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItemInput {
    pub category_id: i64,
    pub label: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateItemInput {
    pub id: i64,
    pub label: Option<String>,
    pub value: Option<String>,
    pub sort_order: Option<i64>,
}

// --- Pool-based functions (testable) ---

pub async fn get_items_by_pool(db: &SqlitePool, category_id: i64) -> Result<Vec<Item>, String> {
    sqlx::query_as::<_, Item>(
        "SELECT id, category_id, label, value, sort_order, created_at, updated_at
         FROM items WHERE category_id = ? ORDER BY sort_order, id",
    )
    .bind(category_id)
    .fetch_all(db)
    .await
    .map_err(|e| e.to_string())
}

pub async fn get_all_items_by_pool(db: &SqlitePool) -> Result<Vec<ItemWithCategory>, String> {
    sqlx::query_as::<_, ItemWithCategory>(
        "SELECT i.id, i.category_id, i.label, i.value, i.sort_order,
                c.name AS category_name, c.sort_order AS category_sort_order
         FROM items i
         JOIN categories c ON c.id = i.category_id
         ORDER BY c.sort_order, c.id, i.sort_order, i.id",
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.to_string())
}

pub async fn create_item_by_pool(db: &SqlitePool, input: CreateItemInput) -> Result<Item, String> {
    let max_order: Option<(i64,)> =
        sqlx::query_as("SELECT COALESCE(MAX(sort_order), -1) FROM items WHERE category_id = ?")
            .bind(input.category_id)
            .fetch_optional(db)
            .await
            .map_err(|e| e.to_string())?;
    let next_order = max_order.map(|r| r.0 + 1).unwrap_or(0);

    let value = input.value.unwrap_or_default();

    let id = sqlx::query(
        "INSERT INTO items (category_id, label, value, sort_order) VALUES (?, ?, ?, ?)",
    )
    .bind(input.category_id)
    .bind(&input.label)
    .bind(&value)
    .bind(next_order)
    .execute(db)
    .await
    .map_err(|e| e.to_string())?
    .last_insert_rowid();

    sqlx::query_as::<_, Item>(
        "SELECT id, category_id, label, value, sort_order, created_at, updated_at
         FROM items WHERE id = ?",
    )
    .bind(id)
    .fetch_one(db)
    .await
    .map_err(|e| e.to_string())
}

pub async fn update_item_by_pool(db: &SqlitePool, input: UpdateItemInput) -> Result<Item, String> {
    let current = sqlx::query_as::<_, Item>(
        "SELECT id, category_id, label, value, sort_order, created_at, updated_at
         FROM items WHERE id = ?",
    )
    .bind(input.id)
    .fetch_optional(db)
    .await
    .map_err(|e| e.to_string())?
    .ok_or_else(|| format!("Item {} not found", input.id))?;

    let label = input.label.unwrap_or(current.label);
    let value = input.value.unwrap_or(current.value);
    let sort_order = input.sort_order.unwrap_or(current.sort_order);

    sqlx::query(
        "UPDATE items SET label = ?, value = ?, sort_order = ?, updated_at = datetime('now') WHERE id = ?",
    )
    .bind(&label)
    .bind(&value)
    .bind(sort_order)
    .bind(input.id)
    .execute(db)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query_as::<_, Item>(
        "SELECT id, category_id, label, value, sort_order, created_at, updated_at
         FROM items WHERE id = ?",
    )
    .bind(input.id)
    .fetch_one(db)
    .await
    .map_err(|e| e.to_string())
}

pub async fn delete_item_by_pool(db: &SqlitePool, id: i64) -> Result<(), String> {
    let result = sqlx::query("DELETE FROM items WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    if result.rows_affected() == 0 {
        return Err(format!("Item {} not found", id));
    }
    Ok(())
}

// --- Tauri commands (thin wrappers) ---

#[tauri::command]
pub async fn get_items(db: State<'_, SqlitePool>, category_id: i64) -> Result<Vec<Item>, String> {
    get_items_by_pool(db.inner(), category_id).await
}

#[tauri::command]
pub async fn get_all_items(db: State<'_, SqlitePool>) -> Result<Vec<ItemWithCategory>, String> {
    get_all_items_by_pool(db.inner()).await
}

#[tauri::command]
pub async fn create_item(
    db: State<'_, SqlitePool>,
    input: CreateItemInput,
) -> Result<Item, String> {
    create_item_by_pool(db.inner(), input).await
}

#[tauri::command]
pub async fn update_item(
    db: State<'_, SqlitePool>,
    input: UpdateItemInput,
) -> Result<Item, String> {
    update_item_by_pool(db.inner(), input).await
}

#[tauri::command]
pub async fn delete_item(db: State<'_, SqlitePool>, id: i64) -> Result<(), String> {
    delete_item_by_pool(db.inner(), id).await
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;
    use crate::commands::categories::{create_category_by_pool, CreateCategoryInput};

    async fn setup_db() -> SqlitePool {
        let db = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("connect sqlite memory");
        sqlx::query(include_str!("../../migrations/001_init.sql"))
            .execute(&db)
            .await
            .expect("run migration 001");
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
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&db)
            .await
            .expect("enable foreign keys");
        db
    }

    async fn create_test_category(
        db: &SqlitePool,
        name: &str,
    ) -> crate::commands::categories::Category {
        create_category_by_pool(
            db,
            CreateCategoryInput {
                name: name.to_string(),
            },
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn get_items_returns_empty_for_category() {
        let db = setup_db().await;
        let cat = create_test_category(&db, "Test").await;
        let items = get_items_by_pool(&db, cat.id).await.unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn create_item_returns_new_item() {
        let db = setup_db().await;
        let cat = create_test_category(&db, "Shortcuts").await;
        let item = create_item_by_pool(
            &db,
            CreateItemInput {
                category_id: cat.id,
                label: "Copy".to_string(),
                value: Some("Cmd+C".to_string()),
            },
        )
        .await
        .unwrap();
        assert_eq!(item.label, "Copy");
        assert_eq!(item.value, "Cmd+C");
        assert_eq!(item.sort_order, 0);
    }

    #[tokio::test]
    async fn create_item_auto_increments_sort_order() {
        let db = setup_db().await;
        let cat = create_test_category(&db, "Shortcuts").await;
        let i1 = create_item_by_pool(
            &db,
            CreateItemInput {
                category_id: cat.id,
                label: "A".to_string(),
                value: None,
            },
        )
        .await
        .unwrap();
        let i2 = create_item_by_pool(
            &db,
            CreateItemInput {
                category_id: cat.id,
                label: "B".to_string(),
                value: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(i1.sort_order, 0);
        assert_eq!(i2.sort_order, 1);
    }

    #[tokio::test]
    async fn update_item_partial_fields() {
        let db = setup_db().await;
        let cat = create_test_category(&db, "Test").await;
        let item = create_item_by_pool(
            &db,
            CreateItemInput {
                category_id: cat.id,
                label: "Old".to_string(),
                value: Some("val".to_string()),
            },
        )
        .await
        .unwrap();

        let updated = update_item_by_pool(
            &db,
            UpdateItemInput {
                id: item.id,
                label: Some("New".to_string()),
                value: None,
                sort_order: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(updated.label, "New");
        assert_eq!(updated.value, "val");
    }

    #[tokio::test]
    async fn delete_item_removes_it() {
        let db = setup_db().await;
        let cat = create_test_category(&db, "Test").await;
        let item = create_item_by_pool(
            &db,
            CreateItemInput {
                category_id: cat.id,
                label: "Del".to_string(),
                value: None,
            },
        )
        .await
        .unwrap();

        delete_item_by_pool(&db, item.id).await.unwrap();

        let items = get_items_by_pool(&db, cat.id).await.unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn delete_item_not_found() {
        let db = setup_db().await;
        let result = delete_item_by_pool(&db, 999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn get_all_items_joins_categories() {
        let db = setup_db().await;
        let cat = create_test_category(&db, "Shortcuts").await;
        create_item_by_pool(
            &db,
            CreateItemInput {
                category_id: cat.id,
                label: "Copy".to_string(),
                value: Some("Cmd+C".to_string()),
            },
        )
        .await
        .unwrap();

        let all = get_all_items_by_pool(&db).await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].category_name, "Shortcuts");
        assert_eq!(all[0].label, "Copy");
    }
}
