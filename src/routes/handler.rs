use actix_web::{get, web, HttpResponse};

#[get("/health_check")]
pub async fn health_check() -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
	let scope = web::scope("/api").service(health_check);

	conf.service(scope);
}
