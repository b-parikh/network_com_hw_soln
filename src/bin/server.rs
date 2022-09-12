use std::net::SocketAddr;

use clap::Parser;

use network_com_hw_soln::utils::{self, Transport};

#[derive(Parser, Debug)]
struct Args {
    /// Type of transport to use to recv messages from the client.
    #[clap(long, value_enum)]
    client_transport_protocol: Transport,

    /// Type of transport to use to send response message to the client.
    #[clap(long, value_enum)]
    server_transport_protocol: Transport,
    /// Socket where the client expects responses from the server. Must be in quotes.
    #[clap(long, value_parser)]
    client_recv_socket_addr: SocketAddr,

    /// Socket where the server expects traffic from the client. Must be in quotes.
    #[clap(long, value_parser)]
    server_recv_socket_addr: SocketAddr,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Create a listener for a request from the client.
    let mut listener = utils::listen(
        &args.server_recv_socket_addr,
        args.client_transport_protocol,
    )?;
    let recvd_msg = utils::recv(&mut listener)?;

    // Respond to client with a copy of the received bytes.
    let mut dialer = utils::dial(
        &args.client_recv_socket_addr,
        args.server_transport_protocol,
    )?;
    utils::send(&recvd_msg, &mut dialer)?;

    Ok(())
}
