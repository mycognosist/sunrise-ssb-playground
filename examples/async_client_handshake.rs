use ssb_crypto::{Keypair, NetworkKey};
use ssb_handshake::HandshakeError;

use async_std::io;
use async_std::net::TcpStream;
use async_std::task;

fn main() -> Result<(), HandshakeError<io::Error>> {
    // https://github.com/sunrise-choir/ssb-handshake
    
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

        let handshake_keys = ssb_handshake::client_side(&mut stream, &net_key, &keypair, &server_pk).await?;
        
        println!("💃 Handshake successful! 💃");

        println!("Connected to peer {:?}", handshake_keys.peer_key.as_base64());

        Ok(())
    })
}
