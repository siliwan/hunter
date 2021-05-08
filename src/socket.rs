use std::io::Result;
use std::os::unix::net::UnixListener;

/// If a socket is requested, it will start one and interact over it.
pub fn start_socket() -> Result<()> {
    let listener = UnixListener::bind("/etc/orion/socket.sock")?;

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {}
            Err(_stream) => {
                eprintln!("Couldn't create IPC-Socket.");
                std::process::exit(-1);
            }
        }
    }

    Ok(())
}
