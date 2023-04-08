use crate::configuration::Settings;
use crate::routes::health_check;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
	port: u16,
	server: Server,
}

impl Application {
	pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
		let address = format!(
			"{}:{}",
			configuration.application.host, configuration.application.port
		);
		let listener = TcpListener::bind(address)?;
		let port = listener.local_addr().unwrap().port();
		let server = run(listener, configuration.application.base_url).await?;

		Ok(Self { port, server })
	}

	pub fn port(&self) -> u16 {
		self.port
	}

	pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
		self.server.await
	}
}

pub struct ApplicationBaseUrl(pub String);

async fn run(listener: TcpListener, base_url: String) -> Result<Server, anyhow::Error> {
	let base_url = Data::new(ApplicationBaseUrl(base_url));
	let server = HttpServer::new(move || {
		App::new()
			.wrap(TracingLogger::default())
			.route("/health_check", web::get().to(health_check))
			.app_data(base_url.clone())
	})
	.listen(listener)?
	.run();

	Ok(server)
}
