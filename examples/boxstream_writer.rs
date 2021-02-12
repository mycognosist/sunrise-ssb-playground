use ssb_boxstream::BoxStream;
use ssb_crypto::NetworkKey;
use ssb_handshake::HandshakeError;

use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

fn main() -> Result<(), HandshakeError<io::Error>> {
    // https://github.com/sunrise-choir/ssb-handshake
    // https://github.com/sunrise-choir/ssb-boxstream

    // start an asynchronous task
    task::block_on(async {
        // read the keypair from our local file
        // note: you will need to change this to match your setup. another option is to generate a
        // unique keypair and write it to file. you will need the public key of this keypair to
        // perform a successful secret handshake when running the `async_client_handshake` example
        let keypair = ssb_keyfile::read_from_path("/home/cordyceps/.ssb/secret").unwrap();

        // set the ssb network key (use the default, main network)
        let net_key = NetworkKey::SSB_MAIN_NET;

        println!("Starting TCP listener...");

        // bind to the given socket address and listen for incoming tcp connections
        let listener = TcpListener::bind("127.0.0.1:8080").await?;

        println!("Listening on {}", listener.local_addr()?);

        // return a stream of incoming connections
        let mut incoming = listener.incoming();

        // iterate over incoming messages from the stream
        while let Some(stream) = incoming.next().await {
            println!("Received transmission from client");

            let mut stream = stream?;

            println!("Attempting secret handshake...");

            let handshake_keys =
                ssb_handshake::server_side(&mut stream, &net_key, &keypair).await?;

            println!("💃 Handshake successful! 💃");

            println!(
                "Connected to peer {:?}",
                handshake_keys.peer_key.as_base64()
            );

            /* attempting to create box stream */

            // split the tcp stream into a reader and writer
            // note: async_std tcp stream does not implement `.split()`.
            // we are cloning the stream here to allow separate access.
            let tcp_reader = stream.clone();
            let tcp_writer = stream;

            println!("Creating a new boxstream");

            // create a new boxstream and split it into a reader and writer
            let (_box_reader, mut box_writer) = BoxStream::new(
                tcp_reader,
                tcp_writer,
                handshake_keys.read_key,
                handshake_keys.read_starting_nonce,
                handshake_keys.write_key,
                handshake_keys.write_starting_nonce,
            )
            .split();

            // test data to be written to the boxstream
            let body = [1, 1, 0, 2, 2, 0, 2, 1, 7, 6, 5, 4, 3, 2, 1, 0];

            println!("Writing to the boxstream");

            // write some data to the box
            box_writer.write_all(&body[0..8]).await?;

            // flush the writer buffer (send the data)
            box_writer.flush().await?;
        }

        Ok(())
    })
}
