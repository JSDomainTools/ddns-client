use core::time::Duration;

use eventsource_client as es;
use futures::{Stream, TryStreamExt};

mod config;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), es::Error> {
	let config = config::Config::read();

	println!("Connecting to {}", config.server());
	let client = es::ClientBuilder::for_url(&format!("{}/update", config.server()))?
		.header("Authorization", config.token())?
		.reconnect(
			es::ReconnectOptions::reconnect(true)
				.retry_initial(true)
				.delay(Duration::from_secs(1))
				.backoff_factor(2)
				.delay_max(Duration::from_secs(60))
				.build(),
		)
		.build();

	let mut stream = events_of(client);

	loop {
		while stream.try_next().await.is_ok() {}
	}
}

fn events_of(client: impl es::Client) -> impl Stream<Item = Result<(), ()>> {
	client
		.stream()
		.map_ok(|_| {})
		.map_err(|err| eprintln!("error streaming events: {:?}", err))
}
