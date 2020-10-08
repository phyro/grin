use cadence::prelude::*;
use cadence::{StatsdClient, UdpMetricSink, DEFAULT_PORT};
/// Metrics over StatsD
use std::net::UdpSocket;

// Create a statsd client that will write to the given host over UDP.
//
// Note that you'll probably want to actually handle any errors creating
// the client when you use it for real in your application. We're just
// using .unwrap() here since this is an example!
lazy_static! {
	// TODO: handle errors
	pub static ref STATSD: StatsdClient = {
		let host = ("localhost", DEFAULT_PORT);
		let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
		let sink = UdpMetricSink::from(host, socket).unwrap();
		let client = StatsdClient::from_sink("grin.node", sink);
		client
	};
}
