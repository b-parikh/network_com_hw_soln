use std::fs::File;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};

use clap::Parser;

use network_com_hw_soln::utils;

#[derive(Parser, Debug)]
struct Args {
    // path of file to send
    #[clap(long, value_parser)]
    stl_file_path: String,

    // The communication mechanism the client uses to send data to the server.
    #[clap(long, value_enum)]
    client_transport_protocol: utils::Transport,

    // The communication mechanism the server should use to return data back to the client.
    #[clap(long, value_enum)]
    server_transport_protocol: utils::Transport,

    // Where the client expects responses from the server.
    #[clap(long, value_parser)]
    client_recv_socket_addr: SocketAddr,

    // Where the server expects traffic from the client.
    #[clap(long, value_parser)]
    server_recv_socket_addr: SocketAddr,
}

fn validate_stl_file(stl_file_path: &str) -> anyhow::Result<PathBuf> {
    let path = Path::new(stl_file_path);
    let mut path_buf = PathBuf::new();
    path_buf.push(path);
    let extension = path_buf
        .extension()
        .expect("File does not contain extension. It must be of type .stl.");
    if extension != "stl" {
        anyhow::bail!("File must have .stl extension");
    }

    Ok(path_buf)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let path = validate_stl_file(&args.stl_file_path)?;

    // Start listening in a new thread before sending a message so the server doesn't see a 
    // `Connection refused` error when responding over a different socket (as the server may 
    // use a different communication protocol).
    let listener_recvd_value = std::thread::spawn(move || -> anyhow::Result<Vec<u8>> {
        let mut listener = utils::listen(
            &args.client_recv_socket_addr,
            args.server_transport_protocol,
        )?;

        utils::recv(&mut listener)
    });

    let _sock_addr = &SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5555);
    let mut dialer = utils::dial(
        &args.server_recv_socket_addr,
        args.client_transport_protocol,
    )?;

    utils::send_file(&path, &mut dialer)?;

    // Get the bytes of the received file.
    let read_buf = listener_recvd_value
        .join()
        .expect("Unable to join the listener recvd value thread")?;

    let save_path = path
        .parent()
        .expect("Unable to get parent of stl file.")
        .join("output.stl");

    // Save to local disk
    let mut f = File::create(save_path)?;
    f.write_all(&read_buf)?;

    Ok(())
}
