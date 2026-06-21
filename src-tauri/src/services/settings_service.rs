use chrono::Utc;
use sqlx::SqlitePool;

use crate::{
    errors::AppResult,
    models::settings::{AppSettingDto, SetAppSettingInput},
};

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

pub async fn list_app_settings(pool: &SqlitePool) -> AppResult<Vec<AppSettingDto>> {
    let rows = sqlx::query_as::<_, AppSettingDto>(
        r#"
        SELECT key, value_json, updated_at
        FROM app_settings
        ORDER BY key ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_app_setting(pool: &SqlitePool, key: String) -> AppResult<Option<AppSettingDto>> {
    let row = sqlx::query_as::<_, AppSettingDto>(
        r#"
        SELECT key, value_json, updated_at
        FROM app_settings
        WHERE key = ?1
        "#,
    )
    .bind(key)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn set_app_setting(
    pool: &SqlitePool,
    input: SetAppSettingInput,
) -> AppResult<AppSettingDto> {
    let now = now_ms();

    sqlx::query(
        r#"
        INSERT INTO app_settings (key, value_json, updated_at)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(key) DO UPDATE SET
          value_json = excluded.value_json,
          updated_at = excluded.updated_at
        "#,
    )
    .bind(&input.key)
    .bind(&input.value_json)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(get_app_setting(pool, input.key)
        .await?
        .expect("setting exists after upsert"))
}
