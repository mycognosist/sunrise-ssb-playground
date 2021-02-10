use ssb_validate;

fn main() {
    // https://sunrise-choir.github.io/ssb-validate/ssb_validate/index.html

    let valid_message_1 = r##"{
        "key": "%/v5mCnV/kmnVtnF3zXtD4tbzoEQo4kRq/0d/bgxP1WI=.sha256",
        "value": {
            "previous": null,
            "author": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
            "sequence": 1,
            "timestamp": 1470186877575,
            "hash": "sha256",
            "content": {
                "type": "about",
                "about": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
                "name": "Piet"
            },
            "signature": "QJKWui3oyK6r5dH13xHkEVFhfMZDTXfK2tW21nyfheFClSf69yYK77Itj1BGcOimZ16pj9u3tMArLUCGSscqCQ==.sig.ed25519"
        },
        "timestamp": 1571140551481
    }"##;

    let valid_message_2 = r##"{
        "key": "%kLWDux4wCG+OdQWAHnpBGzGlCehqMLfgLbzlKCvgesU=.sha256",
        "value": {
            "previous": "%/v5mCnV/kmnVtnF3zXtD4tbzoEQo4kRq/0d/bgxP1WI=.sha256",
            "author": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
            "sequence": 2,
            "timestamp": 1470187292812,
            "hash": "sha256",
            "content": {
                "type": "about",
                "about": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
                "image": {
                    "link": "&MxwsfZoq7X6oqnEX/TWIlAqd6S+jsUA6T1hqZYdl7RM=.sha256",
                    "size": 642763,
                    "type": "image/png",
                    "width": 512,
                    "height": 512
                }
            },
            "signature": "j3C7Us3JDnSUseF4ycRB0dTMs0xC6NAriAFtJWvx2uyz0K4zSj6XL8YA4BVqv+AHgo08+HxXGrpJlZ3ADwNnDw==.sig.ed25519"
        },
        "timestamp": 1571140551485
    }"##;

    // validate the message hash chain
    match ssb_validate::validate_message_hash_chain(
        valid_message_2.as_bytes(),
        Some(valid_message_1.as_bytes()),
    ) {
        Ok(_) => println!("validated"),
        Err(e) => eprintln!("{}", e),
    };
    // validated

    let invalid_message = r##"{ "field": "value" }"##;

    // attempt to validate the message hash chain using one valid and one invalid message
    match ssb_validate::validate_message_hash_chain(
        invalid_message.as_bytes(),
        Some(valid_message_2.as_bytes()),
    ) {
        Ok(_) => println!("validated"),
        Err(e) => eprintln!("{}", e),
    };
    // Message was invalid. Decoding failed with: Message("missing field `key`")

    let valid_msg_value_1 = r##"{
        "previous": null,
        "author": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
        "sequence": 1,
        "timestamp": 1470186877575,
        "hash": "sha256",
        "content": {
            "type": "about",
            "about": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
            "name": "Piet"
        },
        "signature": "QJKWui3oyK6r5dH13xHkEVFhfMZDTXfK2tW21nyfheFClSf69yYK77Itj1BGcOimZ16pj9u3tMArLUCGSscqCQ==.sig.ed25519"
    }"##;

    let valid_msg_value_2 = r##"{
        "previous": "%/v5mCnV/kmnVtnF3zXtD4tbzoEQo4kRq/0d/bgxP1WI=.sha256",
        "author": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
        "sequence": 2,
        "timestamp": 1470187292812,
        "hash": "sha256",
        "content": {
            "type": "about",
            "about": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
            "image": {
                "link": "&MxwsfZoq7X6oqnEX/TWIlAqd6S+jsUA6T1hqZYdl7RM=.sha256",
                "size": 642763,
                "type": "image/png",
                "width": 512,
                "height": 512
            }
        },
        "signature": "j3C7Us3JDnSUseF4ycRB0dTMs0xC6NAriAFtJWvx2uyz0K4zSj6XL8YA4BVqv+AHgo08+HxXGrpJlZ3ADwNnDw==.sig.ed25519"
    }"##;

    // validate the message value hash chain
    match ssb_validate::validate_message_value_hash_chain(
        valid_msg_value_2.as_bytes(),
        Some(valid_msg_value_1.as_bytes()),
    ) {
        Ok(_) => println!("validated"),
        Err(e) => eprintln!("{}", e),
    };
    // This feed is forked. Last known good message was as seq: 1

    let messages = [valid_message_1.as_bytes(), valid_message_2.as_bytes()];

    // If you're passing `None` as the `previous` argument you'll need to give the compiler a hint
    // about the type.

    // validate the message hash chain for a collection of messages
    match ssb_validate::par_validate_message_hash_chain_of_feed::<_, &[u8]>(&messages, None) {
        Ok(_) => println!("validated"),
        Err(e) => eprintln!("{}", e),
    };
    // validated
}
