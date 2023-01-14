use openssl::sha;

fn main() {
    let mut hasher = sha::Sha256::new();

    hasher.update(b"Hello, ");
    hasher.update(b"world");

    let hash = hasher.finish();
    println!("Hashed \"Hello, world\" to {}", hexlify(&hash));
}

fn hexlify(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len());
    for v in data {
        out.push_str(&format!("{v:02x}"));
    }

    out
}
