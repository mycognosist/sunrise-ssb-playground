use ssb_verify_signatures::Error;

fn main() -> Result<(), Error> {
    // https://sunrise-choir.github.io/ssb-verify-signatures/ssb_verify_signatures/fn.verify_message.html

    let key_only_message = r##"{ "key": "%kmXb3MXtBJaNugcEL/Q7G40DgcAkMNTj3yhmxKHjfCM=.sha256" }"##;

    // attempt to verify the signature of the key_only_message
    match ssb_verify_signatures::verify_message(key_only_message.as_bytes()) {
        Ok(_) => println!("verified"),
        Err(e) => eprintln!("{}", e),
    };
    // Error parsing ssb message as json, it is invalid. Errored with: missing field `value` at line 1 column 65

    // See the [`ssb_verify_signatures::Error` enum](https://sunrise-choir.github.io/ssb-verify-signatures/ssb_verify_signatures/enum.Error.html) for a complete listing of possible error types

    let valid_message = r##"{
          "key": "%kmXb3MXtBJaNugcEL/Q7G40DgcAkMNTj3yhmxKHjfCM=.sha256",
          "value": {
            "previous": "%IIjwbJbV3WBE/SBLnXEv5XM3Pr+PnMkrAJ8F+7TsUVQ=.sha256",
            "author": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
            "sequence": 8,
            "timestamp": 1470187438539,
            "hash": "sha256",
            "content": {
                "type": "contact",
                "contact": "@ye+QM09iPcDJD6YvQYjoQc7sLF/IFhmNbEqgdzQo3lQ=.ed25519",
                "following": true,
                "blocking": false
            },
            "signature": "PkZ34BRVSmGG51vMXo4GvaoS/2NBc0lzdFoVv4wkI8E8zXv4QYyE5o2mPACKOcrhrLJpymLzqpoE70q78INuBg==.sig.ed25519"
        },
        "timestamp": 1571140551543
    }"##;

    // verify a single message
    match ssb_verify_signatures::verify_message(valid_message.as_bytes()) {
        Ok(_) => println!("verified"),
        Err(e) => eprintln!("{}", e),
    };
    // verified

    let valid_message_value = r##"{
        "previous": "%IIjwbJbV3WBE/SBLnXEv5XM3Pr+PnMkrAJ8F+7TsUVQ=.sha256",
        "author": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
        "sequence": 8,
        "timestamp": 1470187438539,
        "hash": "sha256",
        "content": {
            "type": "contact",
            "contact": "@ye+QM09iPcDJD6YvQYjoQc7sLF/IFhmNbEqgdzQo3lQ=.ed25519",
            "following": true,
            "blocking": false
        },
        "signature": "PkZ34BRVSmGG51vMXo4GvaoS/2NBc0lzdFoVv4wkI8E8zXv4QYyE5o2mPACKOcrhrLJpymLzqpoE70q78INuBg==.sig.ed25519"
    }"##;

    // verify the `value` fields of a single message
    match ssb_verify_signatures::verify_message_value(valid_message_value.as_bytes()) {
        Ok(_) => println!("verified"),
        Err(e) => eprintln!("{}", e),
    };
    // verified

    // convert message string to bytes for easier batch processing
    let valid_msg = valid_message.as_bytes();
    // verify multiple messages in parallel
    let messages = [valid_msg, valid_msg, valid_msg];

    // verify a collection of messages in parallel
    match ssb_verify_signatures::par_verify_messages(&messages, None) {
        Ok(_) => println!("verified"),
        Err(e) => eprintln!("{}", e),
    };
    // verified

    // convert message value to bytes
    let valid_msg_value = valid_message_value.as_bytes();
    let values = [valid_msg_value, valid_msg_value, valid_msg_value];

    // verify a collection of messages `value` fields in parallel
    match ssb_verify_signatures::par_verify_message_values(&values, None) {
        Ok(_) => println!("verified"),
        Err(e) => eprintln!("{}", e),
    };
    // verified

    Ok(())
}
