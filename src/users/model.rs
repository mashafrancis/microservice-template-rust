use serde::{Deserialize, Serialize};

use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub active: Option<bool>,
	#[serde(rename = "createdAt")]
	pub created_at: Option<chrono::DateTime<chrono::Utc>>,
	#[serde(rename = "updatedAt")]
	pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}