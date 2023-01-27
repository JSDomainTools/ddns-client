pub struct Config {
	server: String,
	token: String,
}

impl Config {
	pub fn server(&self) -> &str {
		&self.server
	}

	pub fn token(&self) -> &str {
		&self.token
	}
}

impl Config {
	pub fn read() -> Self {
		let config_name = "JSDT.toml";
		let config_content = match std::fs::read_to_string(config_name) {
			Ok(c) => c,
			Err(_) => {
				eprintln!("Could not read file `{}`", config_name);
				std::process::exit(1);
			}
		};

		let Ok(config_toml) = toml::from_str::<toml::Table>(&config_content) else {
		eprintln!("Unable to load config from `{}`", config_name);
		std::process::exit(1);
	};

		let toml_ddns = config_toml
			.get("ddns")
			.and_then(|v| v.as_table())
			.expect("missing 'ddns' table");

		let mut config = Config {
			server: toml_ddns
				.get("server")
				.and_then(|v| v.as_str().map(|s| s.to_owned()))
				.expect("missing 'server' string"),
			token: toml_ddns
				.get("token")
				.and_then(|v| v.as_str().map(|s| s.to_owned()))
				.expect("missing 'token' string"),
		};
		#[cfg(debug_assertions)]
		{
			config.server = toml_ddns
				.get("debug")
				.and_then(|v| v.as_table())
				.and_then(|t| t.get("server"))
				.and_then(|v| v.as_str().map(|s| s.to_owned()))
				.unwrap_or(config.server);
		}
		#[cfg(not(debug_assertions))]
		{
			config.server = config.server
		}
		config
	}
}
