use ssb_keyfile;

fn main() {
    let keypair = ssb_keyfile::read_from_path("/home/cordyceps/.ssb/secret").unwrap();

    println!("{}", keypair.public.as_base64());
    println!("{}", keypair.public.as_base64());
}
