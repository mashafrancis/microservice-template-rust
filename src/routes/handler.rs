use crate::startup::AppState;
use crate::users::model::UserModel;
use crate::users::schema::FilterOptions;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/health_check")]
pub async fn health_check() -> HttpResponse {
	const MESSAGE: &str = "Microservice is healthy";
	HttpResponse::Ok().json(json!({ "status": "success", "message": MESSAGE }))
}

#[get("/users")]
pub async fn fetch_users(
	opts: web::Query<FilterOptions>,
	data: web::Data<AppState>,
) -> impl Responder {
	let limit = opts.limit.unwrap_or(10);
	let offset = (opts.page.unwrap_or(1) - 1) * limit;

	let query_result = sqlx::query_as!(
		UserModel,
		"SELECT * FROM users ORDER by id LIMIT $1 OFFSET $2",
		limit as i32,
		offset as i32
	)
	.fetch_all(&data.db)
	.await;

	if query_result.is_err() {
		return HttpResponse::InternalServerError()
			.json(json!({ "status": "error", "message": "Internal server error" }));
	}

	let users = query_result.unwrap();

	let json_response = json!({ "status": "success", "data": users });
	HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
	let scope = web::scope("/api")
		.service(health_check)
		.service(fetch_users);

	conf.service(scope);
}
