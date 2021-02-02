use tempfile;
//use ssb_keyfile;
use ssb_keyfile::{Keypair, KeyFileError};

fn main() -> Result<(), KeyFileError> {
    // https://github.com/sunrise-choir/ssb-keyfile

    //generate a keypair
    let aruna = Keypair::generate();

    println!("{}", aruna.as_base64());
    // vS8uDOK1nU+y3Oxskhfta7AzjbNQ70IpFyoVpzH/IPCzA2COB7U1s7c7NE/2DPjcky67YgZHInOhNWtmqnXVrw==
    
    println!("{}", aruna.public.as_base64());
    // swNgjge1NbO3OzRP9gz43JMuu2IGRyJzoTVrZqp11a8=
    
    // read a keypair from file
    let keypair = ssb_keyfile::read_from_path("/home/cordyceps/.ssb/secret")?;

    println!("{:?}", keypair);
    println!("{}", keypair.public.as_base64());
    // HEqy940T6uB+T+d9Jaa58aNfRzLx9eRWqkZljBmnkmk=

    // create temporary file to demonstrate creating a keypair
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("secret");

    // create a keypair and write to file at the given path
    // path would usually be "{}/.ssb/secret" where {} is the home directory
    let keypair = ssb_keyfile::generate_at_path(&path)?;

    println!("{}", keypair.public.as_base64());
    // Mk1eXJRBE6NSnoxTghzNLw5KGXYkWBZIkEaCZSeXqsY=

    Ok(())
}
