use super::TcpConnection;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener};
use std::sync::{Arc, mpsc};
use std::time::Duration;

#[test]
fn cancelling_a_timed_connect_reaches_peer_before_transport_drop() {
    for trial in 0..200 {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap().to_string();
        let (ended, observed) = mpsc::channel();
        std::thread::scope(|scope| {
            let peer = scope.spawn(move || -> std::io::Result<()> {
                let (mut stream, _) = listener.accept()?;
                stream.set_read_timeout(Some(Duration::from_secs(2)))?;
                stream.write_all(&[7])?;
                let mut byte = [0];
                let read = stream.read(&mut byte)?;
                ended.send(read).unwrap();
                Ok(())
            });
            let mut registered = None;
            let mut register = |socket: &Arc<std::net::TcpStream>| {
                registered = Some(Arc::clone(socket));
                Ok(())
            };
            let mut connection = TcpConnection::connect_registered(
                &address,
                Duration::from_secs(2),
                None,
                &mut register,
                &mut || Ok(()),
            )
            .unwrap();
            let mut marker = [0];
            connection.stream.read_exact(&mut marker).unwrap();
            assert_eq!(marker, [7]);
            let shutdown = registered.unwrap().shutdown(Shutdown::Both);
            assert!(shutdown.is_ok(), "trial {trial}: registered socket shutdown {shutdown:?}");
            assert_eq!(
                observed.recv_timeout(Duration::from_secs(2)).unwrap(),
                0,
                "trial {trial}: cancellation must reach the peer while transport is retained"
            );
            drop(connection);
            peer.join().unwrap().unwrap();
        });
    }
}
