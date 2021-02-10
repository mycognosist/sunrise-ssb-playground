use ssb_multiformats::multihash::{DecodeLegacyError, Multihash};

fn main() -> Result<(), DecodeLegacyError> {
    // https://github.com/sunrise-choir/ssb-multiformats/blob/master/src/multihash.rs

    // convert `str` to a byte slice (`[u8]`) with `as_bytes()`
    let msg_ref = "%x60lINqpVU9Fw5cdNq/7raSy/N2zUs8NT9TLsXu5qSQ=.sha256".as_bytes();

    // attempt to parse a message reference with legacy encoding
    match Multihash::from_legacy(&msg_ref) {
        Ok(_) => println!("is msg"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
    // is msg

    let blob_ref = "&6HcP+bp6kG/pctvcC2XPQrcKyPOY9XAN7igp/opMjuE=.sha256".as_bytes();

    // attempt to parse a blob reference with legacy encoding.
    // `from_legacy()` returns `Ok(Multihash, &[u8])` on success, where `Multihash` is an enum
    // with `Message([u8; 32])` and `Blob([u8; 32])` variants.
    let blob_multihash = Multihash::from_legacy(&blob_ref)?;
    println!("{:?}", blob_multihash);
    // (Blob([232, 119, 15, 249, 186, 122, 144, 111, 233, 114, 219, 220, 11, 101, 207, 66, 183, 10, 200, 243, 152, 245, 112, 13, 238, 40, 41, 254, 138, 76, 142, 225]), [])

    let feed_ref = "@nUtgCIpqOsv6k5mnWKA4JeJVkJTd9Oz2gmv6rojQeXU=.ed25519".as_bytes();

    // attempt to parse a legacy encoding into a `Multifeed`
    match Multihash::from_legacy(&feed_ref) {
        Ok(_) => println!("is msg or blob"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
    // Error: Sigil

    Ok(())
}
