use ssb_boxstream::BoxStream;
use ssb_crypto::NetworkKey;
use ssb_handshake::HandshakeError;
use ssb_packetstream::*;

use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

use futures::sink::SinkExt;

fn main() -> Result<(), HandshakeError<io::Error>> {
    // https://github.com/sunrise-choir/ssb-packetstream
    // https://docs.rs/ssb-packetstream/0.2.1/ssb_packetstream/

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

            // split the tcp stream into a reader and writer
            // note: async_std tcp stream does not implement `.split()`.
            // we are cloning the stream here to allow separate access.
            let tcp_reader = stream.clone();
            let tcp_writer = stream;

            println!("Creating a new boxstream");

            // create a new boxstream and split it into a reader and writer
            let (_box_reader, box_writer) = BoxStream::new(
                tcp_reader,
                tcp_writer,
                handshake_keys.read_key,
                handshake_keys.read_starting_nonce,
                handshake_keys.write_key,
                handshake_keys.write_starting_nonce,
            )
            .split();

            println!("Creating a new packet sink");

            // create a new packet sink (writer)
            let mut packet_sink = PacketSink::new(box_writer);

            // create a packet to send (write to sink)
            let p = Packet::new(
                IsStream::Yes,
                IsEnd::No,
                BodyType::Binary,
                12345,
                vec![1, 2, 3, 4, 5],
            );

            // send the packet
            packet_sink.send(p).await.unwrap();

            println!("Packet sent");
        }

        Ok(())
    })
}
