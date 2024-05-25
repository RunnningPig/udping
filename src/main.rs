use clap::Parser;
use std::net::IpAddr;
use std::time::Duration;

pub mod udping;
pub mod udpong;
use crate::udping::UdpPingClient;
use crate::udpong::UdpPongServer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Run in pong mode
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    server: bool,

    /// Pong server port to listen on/connect to
    #[arg(short, long, default_value_t = 23333)]
    port: u16,

    /// Time to wait for a response, in seconds.
    #[arg(long, default_value_t = 3)]
    timeout: u64,

    #[arg(required_unless_present = "server")]
    host: Option<String>,
}

fn main() -> std::io::Result<()> {
    human_panic::setup_panic!();

    let args = Args::parse();

    if args.server {
        let listen_addr: (IpAddr, u16) = ("0.0.0.0".parse().unwrap(), args.port);
        let server = UdpPongServer::new();
        server.run(listen_addr)?;
    } else {
        let remote_addr = (args.host.unwrap(), args.port);
        let mut client = UdpPingClient::new();
        client.set_timeout(Duration::new(args.timeout, 0));
        client.run(remote_addr)?;
    }
    Ok(())
}
