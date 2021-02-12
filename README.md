# Sunrise SSB Playground

Fun with the [Sunrise Choir](https://github.com/sunrise-choir) Rust SSB code.

Contains code examples corresponding to the documentation at [dev.scuttlebutt.nz](https://dev.scuttlebutt.nz/#/rust/sunrise-choir).

_Work in progress._

## Intention

I wanted to compile a set of basic code examples which demonstrate how to use the building blocks of the Secure Scuttlebutt Protocol in Rust (Sunrise Choir implementation). In doing so, I wanted to make it easier for myself and peers to write code for existing Scuttlebutt applications and to craft beautiful new applications. I have had a lot of fun and learned a great deal through this process.

Please be aware that I do not adhere to Rust best-practices throughout the examples; there are, for example, quite a few naughty `unwraps()` which should be handled more elegantly. I have mostly made this trade-off to keep the code concise and to leave implementation details to the reader.

## Usage

Print a list of the available examples:

```bash
cargo run --example

Available examples:
    boxstream_reader
    boxstream_writer
    handshake_client
    handshake_server
    keyfile
    network_keys
    packetstream_reader
    packetstream_writer
    parse_ref_to_multifeed
    parse_ref_to_multihash
    sign_message
    validate_message
    verify_message
    verify_signature
```

Run one of the examples:

```bash
cargo run --example sign_message

"N6YTtLv2bZ9vdn37Mnddto3tq1mXwLviOr5ONE2igDBibxsSXeynSC9SA7THR7gaZNzgRdZd5rIEul53qOc2CA=="
```

Note: the output of most of the examples is very minimal, with the exception of the `handshake`, `boxstream` and `packetstream` examples which offer more verbose output. Code comments are included in the source for all examples.

## Order

If following-along with the corresponding guide at [dev.scuttlebutt.nz](https://dev.scuttlebutt.nz/#/rust/sunrise-choir), the order of the examples is as follows:

```bash
    keyfile
    sign_message
    verify_signature
    verify_message
    validate_message
    parse_ref_to_multifeed
    parse_ref_to_multihash
    network_keys
    handshake_client
    handshake_server
    boxstream_reader
    boxstream_writer
    packetstream_reader
    packetstream_writer
```

## Correspondence

You are welcome to open issues on this repo if you have any questions or suggestions for improvement. I may not know the answer to your question, but we can seek out a solution(s) together.
