use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
	port: u16,
	server: Server,
}

impl Application {
	pub async fn build() -> Result<Self, anyhow::Error> {
		let address = format!("{}:{}", " ", 8000);
		let listener = TcpListener::bind(address)?;
		let port = listener.local_addr().unwrap().port();
		let server = run(listener).await?;

		Ok(Self { port, server })
	}

	pub fn port(&self) -> u16 {
		self.port
	}

	pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
		self.server.await
	}
}

async fn run(listener: TcpListener) -> Result<Server, anyhow::Error> {
	let server = HttpServer::new(|| App::new().wrap(TracingLogger).service(health_check))
		.listen(listener)?
		.run();

	Ok(server)
}