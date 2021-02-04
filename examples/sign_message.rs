use ssb_keyfile::{KeyFileError, Keypair};

use serde_json::json;

fn main() -> Result<(), KeyFileError> {
    // https://docs.rs/ssb-crypto/0.2.2/ssb_crypto/struct.Keypair.html
    // https://docs.rs/ssb-crypto/0.2.2/ssb_crypto/struct.Signature.html

    // generate a keypair
    let aruna = Keypair::generate();

    // create an example object with json encoding
    let example_object = json!({ "type": "example" });

    // convert the json object to a string
    let example_string = example_object.to_string();

    // generate a signature for the object
    let signature = aruna.sign(&example_string.as_bytes());

    // print a base64 representation of the signature
    println!("{:?}", signature.as_base64());

    // "o5w5E3Kt2AmJlwx4mocLz4V622m5Y0C3jirs1xTDoHdxvAXbfZMMJ7PfEE5ZV6eLWpI3HyhtmiqyUlRuMSZsBw=="

    Ok(())
}
