use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
	pub page: Option<usize>,
	pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
	pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
	pub username: String,
	pub email: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserSchema {
	pub username: Option<String>,
	pub email: Option<String>,
	pub active: Option<bool>,
}
