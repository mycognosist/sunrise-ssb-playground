use ssb_multiformats::multifeed::{DecodeLegacyError, Multifeed};

fn main() -> Result<(), DecodeLegacyError> {
    // https://github.com/sunrise-choir/ssb-multiformats/blob/master/src/multifeed.rs

    // convert `str` to a byte slice (`[u8]`) with `as_bytes()`
    let feed_ref = "@nUtgCIpqOsv6k5mnWKA4JeJVkJTd9Oz2gmv6rojQeXU=.ed25519".as_bytes();

    // check if given ssb reference is a feed
    if Multifeed::from_legacy(&feed_ref).is_ok() {
        println!("is feed")
    };

    // attempt to parse a legacy encoding into a `Multifeed`.
    // `from_legacy()` returns `Ok(Multifeed, &[u8])` on success, where `Multifeed` is an enum with
    // a single variant `Multikey(Multikey)`
    let feed = Multifeed::from_legacy(&feed_ref)?;
    println!("{:?}", feed);
    // (Multikey(Ed25519(PublicKey([157, 75, 96, 8, 138, 106, 58, 203, 250, 147, 153, 167, 88, 160, 56, 37, 226, 85, 144, 148, 221, 244, 236, 246, 130, 107, 250, 174, 136, 208, 121, 117]))), [])

    let msg_ref = "%x60lINqpVU9Fw5cdNq/7raSy/N2zUs8NT9TLsXu5qSQ=.sha256".as_bytes();

    // attempt to parse a message reference with legacy encoding
    match Multifeed::from_legacy(&msg_ref) {
        Ok(_) => println!("is feed"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
    // Error: UnknownKind

    // `UnknownKind` is returned if the input does not start with the `"@"` sigil.
    // a second error variant (`Multikey(multikey::DecodeLegacyError)`) is returned if the inner
    // multikey decoding fails

    let blob_ref = "&6HcP+bp6kG/pctvcC2XPQrcKyPOY9XAN7igp/opMjuE=.sha256".as_bytes();

    // attempt to parse a blob reference with legacy encoding
    Multifeed::from_legacy(&blob_ref)?;
    // Error: UnknownKind

    Ok(())
}
