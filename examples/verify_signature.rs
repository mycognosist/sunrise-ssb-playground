use ssb_keyfile::Keypair;
use ssb_verify_signatures::Error;

use serde_json::json;

fn main() -> Result<(), Error> {
    // https://docs.rs/ssb-crypto/0.2.2/ssb_crypto/struct.PublicKey.html#method.verify

    // generate a keypair
    let aruna = Keypair::generate();

    // create an example object with json encoding
    let example_object = json!({ "type": "example" });

    // convert the json object to a string
    let example_string = example_object.to_string();

    // sign the string representation of the json object
    let signature = aruna.sign(&example_string.as_bytes());

    // attempt to verify the signed object with aruna's public key
    let is_valid = aruna.public.verify(&signature, &example_string.as_bytes());
    println!("{}", is_valid);
    // true

    // generate a second keypair
    let benedict = Keypair::generate();

    // attempt to verify the signed object with benedict's public key
    let is_valid_b = benedict
        .public
        .verify(&signature, &example_string.as_bytes());
    println!("{}", is_valid_b);
    // false

    Ok(())
}
