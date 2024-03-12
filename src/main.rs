use a2s::A2SClient;
use serde::Deserialize;
use std::{
	fs::{self, ReadDir},
	path::PathBuf,
	process::Command,
	thread,
	time::Duration,
};

mod gmod_server;
use gmod_server::GmodServer;

fn update_addon(addon: PathBuf) {
	let dir_str = match addon.to_str() {
		Some(v) => v,
		None => return,
	};

	let result = Command::new("git").args(["-C", dir_str, "pull"]).status();

	match result {
		Ok(_) => (),
		Err(e) => eprintln!("Git command failed! {}", e),
	}
}

fn update_addons(addons: ReadDir) {
	for addon in addons {
		match addon {
			Ok(v) => {
				if !v.path().is_dir() {
					continue;
				}

				update_addon(v.path())
			}
			Err(_) => (),
		}
	}
}

#[derive(Deserialize)]
struct ServerConfig {
	address: String,
	max_pings: usize,
	maxplayers: u8,
	path: String,
	token: Option<String>,
	workshop: Option<String>,
}

fn main() {
	let data = fs::read("config.json").unwrap();
	let strdata = String::from_utf8(data).unwrap();
	let config = serde_json::from_str::<ServerConfig>(&strdata).unwrap();

	let mut server = GmodServer::new(&config.path).set_maxplayers(config.maxplayers);

	if let Some(token) = config.token {
		server = server.set_token(token);
	}

	if let Some(workshop) = config.workshop {
		server = server.set_workshop(workshop);
	}

	let mut ping_counter = 0;

	let a2s_client = A2SClient::new().unwrap();

	loop {
		let mut server_online = false;

		println!("Updating git repos :3");
		println!("{}/garrysmod/addons", config.path);
		let addons = fs::read_dir(format!("{}/garrysmod/addons", config.path)).unwrap();
		update_addons(addons);

		println!("Starting server =3 ...");
		server.start().expect("Failed to start server process");

		loop {
			match a2s_client.info(&config.address) {
				Ok(_) => {
					if !server_online {
						server_online = true;
						println!("Server online x3 !!!");
					}

					ping_counter = 0;
				}
				Err(_) => {
					if !server_online {
						continue;
					}

					if !server.is_running() {
						println!(r"Server process has exited (=^･ω･^=), restarting ...");
						break;
					}

					ping_counter += 1;
					println!("Ping failed ({ping_counter}/{}) :3c", config.max_pings);

					if ping_counter >= config.max_pings {
						println!("Server failed to respond after '{ping_counter}' pings ...");
						println!("Restarting server :3 ...");

						break;
					}
				}
			}

			thread::sleep(Duration::from_secs(30));
		}
	}
}
