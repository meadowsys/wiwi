use tokio::net::TcpSocket;
use tokio::io::AsyncRead;

pub struct ClientBuilder {}
pub struct Client {}

pub struct ServerBuilder {}
pub struct Server {}

// pub struct Connection {}

pub struct ConnectOpts {

}

impl Client {
	pub fn builder() -> ClientBuilder {
		todo!()
	}

	pub fn connect(url: &str) {}
}

impl Server {
	pub fn builder() -> ServerBuilder {
		todo!()
	}
}

use url::Url;
