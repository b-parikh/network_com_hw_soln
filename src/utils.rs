use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;

use anyhow::Context;
use clap::ValueEnum;
use nng;

#[derive(Clone, Debug, ValueEnum)]
pub enum Transport {
    Tcp,
    Nng,
}

pub enum ListenerTransportSocket {
    Tcp(TcpStream),
    Nng(nng::Socket),
}

pub enum DialerTransportSocket {
    Tcp(TcpStream),
    Nng(nng::Socket),
}

pub fn listen(
    socket_addr: &SocketAddr,
    transport: Transport,
) -> anyhow::Result<ListenerTransportSocket> {
    match transport {
        Transport::Tcp => {
            let listener = TcpListener::bind(socket_addr)?;
            let (stream, _) = listener.accept()?;
            Ok(ListenerTransportSocket::Tcp(stream))
        }
        Transport::Nng => {
            // Only support Pair protocol for now.
            let socket = nng::Socket::new(nng::Protocol::Pair1)?;
            let mut url = String::from("tcp://");
            url.push_str(&socket_addr.to_string());

            socket.listen(&url)?;
            Ok(ListenerTransportSocket::Nng(socket))
        }
    }
}

pub fn recv(listener_transport: &mut ListenerTransportSocket) -> anyhow::Result<Vec<u8>> {
    match listener_transport {
        ListenerTransportSocket::Tcp(stream) => {
            let mut buf: Vec<u8> = Vec::new();
            stream.read_to_end(&mut buf)?;
            Ok(buf)
        }
        ListenerTransportSocket::Nng(listener) => Ok(listener.recv()?.as_slice().to_vec()),
    }
}

pub fn dial(
    socket_addr: &SocketAddr,
    transport: Transport,
) -> anyhow::Result<DialerTransportSocket> {
    match transport {
        Transport::Tcp => {
            let stream = TcpStream::connect(socket_addr)?;
            Ok(DialerTransportSocket::Tcp(stream))
        }
        Transport::Nng => {
            // Only support Pair protocol for now.
            let mut url = String::from("tcp://");
            url.push_str(&socket_addr.to_string());
            let socket = nng::Socket::new(nng::Protocol::Pair1)?;
            socket.dial(&url)?;
            Ok(DialerTransportSocket::Nng(socket))
        }
    }
}

pub fn send(buf: &[u8], dialer_transport: &mut DialerTransportSocket) -> anyhow::Result<()> {
    match dialer_transport {
        DialerTransportSocket::Tcp(stream) => {
            stream.write_all(buf)?;
            // `read_to_end` is used in the `recv` implementation. It expects an EOF to return to the caller.
            // Sending a shutdown message sends an EOF to the receiver.
            stream.shutdown(std::net::Shutdown::Write)?;
        }
        DialerTransportSocket::Nng(dialer) => {
            let mut msg = nng::Message::new();
            msg.push_back(buf);

            // The `send()` call returns `(nng::Message, nng::Error)`, and `nng::Error` doesn't
            // implement the `stdError` trait
            // (https://doc.rust-lang.org/nightly/core/error/trait.Error.html).
            // However, `anyhow::Result<T, E>` expects E to be an Error type that does implement the
            // `stdError` trait. For this reason, we need to change the returned error into an `anyhow::Error`.
            dialer.send(msg).map_err(|(_m, e)| anyhow::Error::new(e))?;
        }
    }
    Ok(())
}

pub fn send_file(
    path: &PathBuf,
    dialer_transport: &mut DialerTransportSocket,
) -> anyhow::Result<()> {
    let size_bytes = std::fs::metadata(&path)?.len();
    // PANIC: Safely convert from u64 -> usize. If usize != 64 bits on the platform, panic! and
    // kill the thread.
    let size_bytes = usize::try_from(size_bytes).unwrap();
    let mut file = File::open(&path)
        .with_context(|| format!("Unable to open .stl file at {}", &path.display()))?;

    let mut buf: Vec<u8> = Vec::new();
    buf.resize(size_bytes, 0);
    let _ = file.read(&mut buf)?;
    send(&buf, dialer_transport)
}
