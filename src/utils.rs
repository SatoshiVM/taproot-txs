use sha2::Digest;
use sha2::Sha256;

pub fn sha256(input: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hex::encode(hasher.finalize())
}

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex::decode(hex).expect("Decoding failed")
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn get_rand(size: usize) -> String {
    let random_bytes: Vec<u8> = (0..size).map(|_| rand::random::<u8>()).collect();
    bytes_to_hex(&random_bytes)
}

pub fn and(a: bool, b: bool) -> bool {
    a && b
}

pub fn xor(a: bool, b: bool) -> bool {
    a ^ b
}

pub fn inv(a: bool) -> bool {
    !a
}

#[test]
pub fn test() {
    let output = sha256(&[5u8, 10u8]);
    println!("{:?}", output);
}
