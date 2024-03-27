//! Websocket implementation

super::runtime_selection_compile_check!("ws");

use ::tokio::net::TcpSocket;

pub struct ClientBuilder {}
pub struct Client {}

pub struct ServerBuilder {}
pub struct Server {}

// pub struct Connection {}

impl Client {
	pub fn builder() -> ClientBuilder {
		todo!()
	}
}

impl Server {
	pub fn builder() -> ServerBuilder {
		todo!()
	}
}
