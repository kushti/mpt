// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::sync::Arc;
use io::PanicHandler;
use rpc_apis;
use ethcore::client::Client;
use ethsync::SyncProvider;
use helpers::replace_home;

#[derive(Debug, PartialEq, Clone)]
pub struct Configuration {
	pub enabled: bool,
	pub interface: String,
	pub port: u16,
	pub hosts: Option<Vec<String>>,
	pub user: Option<String>,
	pub pass: Option<String>,
	pub dapps_path: String,
}

impl Default for Configuration {
	fn default() -> Self {
		Configuration {
			enabled: true,
			interface: "127.0.0.1".into(),
			port: 8080,
			hosts: Some(Vec::new()),
			user: None,
			pass: None,
			dapps_path: replace_home("$HOME/.parity/dapps"),
		}
	}
}

pub struct Dependencies {
	pub panic_handler: Arc<PanicHandler>,
	pub apis: Arc<rpc_apis::Dependencies>,
	pub client: Arc<Client>,
	pub sync: Arc<SyncProvider>,
}

pub fn new(configuration: Configuration, deps: Dependencies) -> Result<Option<WebappServer>, String> {
	if !configuration.enabled {
		return Ok(None);
	}

	let signer_port = deps.apis.signer_service.port();
	let url = format!("{}:{}", configuration.interface, configuration.port);
	let addr = try!(url.parse().map_err(|_| format!("Invalid Webapps listen host/port given: {}", url)));

	let auth = configuration.user.as_ref().map(|username| {
		let password = configuration.pass.as_ref().map_or_else(|| {
			use rpassword::read_password;
			println!("Type password for WebApps server (user: {}): ", username);
			let pass = read_password().unwrap();
			println!("OK, got it. Starting server...");
			pass
		}, |pass| pass.to_owned());
		(username.to_owned(), password)
	});

	Ok(Some(try!(setup_dapps_server(deps, configuration.dapps_path, &addr, configuration.hosts, auth, signer_port))))
}

pub use self::server::WebappServer;
pub use self::server::setup_dapps_server;

#[cfg(not(feature = "dapps"))]
mod server {
	use super::Dependencies;
	use std::net::SocketAddr;

	pub struct WebappServer;
	pub fn setup_dapps_server(
		_deps: Dependencies,
		_dapps_path: String,
		_url: &SocketAddr,
		_allowed_hosts: Option<Vec<String>>,
		_auth: Option<(String, String)>,
		_signer_port: Option<u16>,
	) -> Result<WebappServer, String> {
		Err("Your Parity version has been compiled without WebApps support.".into())
	}
}

#[cfg(feature = "dapps")]
mod server {
	use super::Dependencies;
	use std::sync::Arc;
	use std::net::SocketAddr;
	use std::io;
	use util::{Bytes, Address, U256};

	use ethcore::transaction::{Transaction, Action};
	use ethcore::client::{Client, BlockChainClient, BlockID};

	use rpc_apis;
	use ethcore_rpc::is_major_importing;
	use ethcore_dapps::ContractClient;

	pub use ethcore_dapps::Server as WebappServer;

	pub fn setup_dapps_server(
		deps: Dependencies,
		dapps_path: String,
		url: &SocketAddr,
		allowed_hosts: Option<Vec<String>>,
		auth: Option<(String, String)>,
		signer_port: Option<u16>,
	) -> Result<WebappServer, String> {
		use ethcore_dapps as dapps;

		let mut server = dapps::ServerBuilder::new(
			dapps_path,
			Arc::new(Registrar { client: deps.client.clone() })
		);
		let sync = deps.sync.clone();
		let client = deps.client.clone();
		server.with_sync_status(Arc::new(move || is_major_importing(Some(sync.status().state), client.queue_info())));
		server.with_signer_port(signer_port);

		let server = rpc_apis::setup_rpc(server, deps.apis.clone(), rpc_apis::ApiSet::UnsafeContext);
		let start_result = match auth {
			None => {
				server.start_unsecured_http(url, allowed_hosts)
			},
			Some((username, password)) => {
				server.start_basic_auth_http(url, allowed_hosts, &username, &password)
			},
		};

		match start_result {
			Err(dapps::ServerError::IoError(err)) => match err.kind() {
				io::ErrorKind::AddrInUse => Err(format!("WebApps address {} is already in use, make sure that another instance of an Ethereum client is not running or change the address using the --dapps-port and --dapps-interface options.", url)),
				_ => Err(format!("WebApps io error: {}", err)),
			},
			Err(e) => Err(format!("WebApps error: {:?}", e)),
			Ok(server) => {
				server.set_panic_handler(move || {
					deps.panic_handler.notify_all("Panic in WebApp thread.".to_owned());
				});
				Ok(server)
			},
		}
	}

	struct Registrar {
		client: Arc<Client>,
	}

	impl ContractClient for Registrar {
		fn registrar(&self) -> Result<Address, String> {
			self.client.additional_params().get("registrar")
				 .ok_or_else(|| "Registrar not defined.".into())
				 .and_then(|registrar| {
					 registrar.parse().map_err(|e| format!("Invalid registrar address: {:?}", e))
				 })
		}

		fn call(&self, address: Address, data: Bytes) -> Result<Bytes, String> {
			let from = Address::default();
			let transaction = Transaction {
				nonce: self.client.latest_nonce(&from),
				action: Action::Call(address),
				gas: U256::from(50_000_000),
				gas_price: U256::default(),
				value: U256::default(),
				data: data,
			}.fake_sign(from);

			self.client.call(&transaction, BlockID::Latest, Default::default())
				.map_err(|e| format!("{:?}", e))
				.map(|executed| {
					executed.output
				})
		}
	}
}
