use ssb_crypto::NetworkKey;
use ssb_handshake::HandshakeError;

use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

fn main() -> Result<(), HandshakeError<io::Error>> {
    // https://github.com/sunrise-choir/ssb-handshake
    
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

            let handshake_keys = ssb_handshake::server_side(&mut stream, &net_key, &keypair).await?;
            
            println!("💃 Handshake successful! 💃");

            println!("Connected to peer {:?}", handshake_keys.peer_key.as_base64());
        }

        Ok(())
    })
}

/*
async fn server_handshake(mut stream: TcpStream) -> Result<(), HandshakeError<io::Error>> {
    println!("Accepted from: {}", stream.peer_addr()?);

    //let keypair = Keypair::generate();
    
    // READ SECRET FROM FILE _ DON'T PUSH !
    let keypair = ssb_keyfile::read_from_path("/home/cordyceps/.ssb/secret").unwrap();
    let net_key = NetworkKey::SSB_MAIN_NET;

    let _handshake_keys = ssb_handshake::server_side(&mut stream, &net_key, &keypair).await?;

    println!("SUCCESS!");

    Ok(())
}

//fn main() -> io::Result<()> {
fn main() -> Result<(), HandshakeError<io::Error>> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                server_handshake(stream).await;
            });
        }
        Ok(())
    })
}
*/
