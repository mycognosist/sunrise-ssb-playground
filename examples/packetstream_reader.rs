use ssb_boxstream::BoxStream;
use ssb_crypto::{Keypair, NetworkKey};
use ssb_handshake::HandshakeError;
use ssb_packetstream::*;

use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;

use futures::sink::SinkExt;

//use std::net::Shutdown;

fn main() -> Result<(), HandshakeError<io::Error>> {
    // start an asynchronous task
    task::block_on(async {
        // generate a keypair for the client
        let keypair = Keypair::generate();

        // read the server's keypair from our local secret file
        // note: this is purely for demonstration purposes, since we are running server and client
        // on the same machine. in a proper implementation, we would have to source the public key
        // of the server from some other discovery mechanism (for example, udp broadcast messages
        // on the local network)
        let server_keypair = ssb_keyfile::read_from_path("/home/cordyceps/.ssb/secret").unwrap();
        let server_pk = server_keypair.public;

        // set the ssb network key (use the default, main network)
        let net_key = NetworkKey::SSB_MAIN_NET;

        println!("Attempting TCP connection...");

        // create an asynchronous tcp stream and attempt to connect to the given ip address and port
        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

        println!("Connected to {}", &stream.peer_addr()?);

        println!("Attempting secret handshake...");

        let handshake_keys =
            ssb_handshake::client_side(&mut stream, &net_key, &keypair, &server_pk).await?;

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
        let (box_reader, box_writer) = BoxStream::new(
            tcp_reader,
            tcp_writer,
            handshake_keys.read_key,
            handshake_keys.read_starting_nonce,
            handshake_keys.write_key,
            handshake_keys.write_starting_nonce,
        )
        .split();

        println!("Creating a new packet stream");

        // create a new packet stream (reader)
        let mut packet_stream = PacketStream::new(box_reader);

        println!("Creating a new packet sink");

        // create a new packet sink (writer)
        let mut packet_sink = PacketSink::new(box_writer);

        // attempt to receive a packet (read from stream)
        let packet = packet_stream.next().await.unwrap().unwrap();

        println!("Packet received: {:?} from {:?}", &packet.body, packet.id);

        // check if data packet is part of a stream
        println!("IsStream: {}", packet.is_stream());
        
        // check if data packet is a 'goodbye message'
        println!("IsEnd: {}", packet.is_end());
        
        println!("Sending goodbye message...");
        
        packet_sink.close().await.unwrap();

        Ok(())
    })
}
