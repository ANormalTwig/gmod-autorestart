use std::{
	io::Result,
	process::{Child, Command},
};

pub struct GmodServer {
	maxplayers: u8,
	path: String,
	token: Option<String>,
	workshop: Option<String>,
	process: Option<Child>,
}

impl GmodServer {
	pub fn new(path: &String) -> GmodServer {
		GmodServer {
			path: path.clone(),
			maxplayers: 16,
			token: None,
			workshop: None,
			process: None,
		}
	}

	pub fn set_maxplayers(mut self, maxplayers: u8) -> GmodServer {
		self.maxplayers = maxplayers;
		return self;
	}

	pub fn set_token(mut self, token: String) -> GmodServer {
		self.token = Some(token);
		return self;
	}

	pub fn set_workshop(mut self, workshop: String) -> GmodServer {
		self.workshop = Some(workshop);
		return self;
	}

	pub fn is_running(&mut self) -> bool {
		match &mut self.process {
			Some(child) => match child.try_wait() {
				Ok(opt) => match opt {
					Some(_) => (),
					None => return true,
				},
				Err(_) => (),
			},
			None => (),
		}

		return false;
	}

	pub fn start(&mut self) -> Result<()> {
		match &mut self.process {
			Some(v) => {
				v.kill()?;
				v.wait()?;
			}
			None => (),
		};

		let mut args = Vec::from(vec!["-norestart", "+map", "gm_flatgrass"]);

		let maxplayers_str = self.maxplayers.to_string();

		args.push("+maxplayers");
		args.push(maxplayers_str.as_str());

		if let Some(token) = &self.token {
			args.push("+sv_setsteamaccount");
			args.push(token);
		}

		if let Some(workshop) = &self.workshop {
			args.push("+host_workshop_collection");
			args.push(workshop);
		}

		self.process = Some(
			Command::new(format!("{}/srcds_run", self.path))
				.args(args)
				.spawn()?,
		);

		Ok(())
	}
}
