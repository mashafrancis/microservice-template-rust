use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::health_check;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
	port: u16,
	server: Server,
}

impl Application {
	pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
		let connection_pool = get_connection_pool(&configuration.database);
		let address = format!(
			"{}:{}",
			configuration.application.host, configuration.application.port
		);
		let listener = TcpListener::bind(address)?;
		let port = listener.local_addr().unwrap().port();
		let server = run(
			listener,
			configuration.application.base_url,
			connection_pool,
		)
		.await?;

		Ok(Self { port, server })
	}

	pub fn port(&self) -> u16 {
		self.port
	}

	pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
		self.server.await
	}
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
	PgPoolOptions::new()
		.acquire_timeout(std::time::Duration::from_secs(2))
		.connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);

async fn run(
	listener: TcpListener,
	base_url: String,
	db_pool: PgPool,
) -> Result<Server, anyhow::Error> {
	let base_url = Data::new(ApplicationBaseUrl(base_url));
	let db_pool = Data::new(db_pool);
	let server = HttpServer::new(move || {
		App::new()
			.wrap(TracingLogger::default())
			.route("/health_check", web::get().to(health_check))
			.app_data(base_url.clone())
			.app_data(db_pool.clone())
	})
	.listen(listener)?
	.run();

	Ok(server)
}
