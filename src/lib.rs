mod bitcommitmenttx;
mod circuits;
#[allow(dead_code)]
mod gate_templates;
mod timelock;
pub mod utils;
mod verificationtx;

pub use bitcommitmenttx::generate_bit_commitment_address;
pub use verificationtx::generate_verification_address;

#[test]
fn test_savm_taproot_tx_creation() {
    let circuit = "4 7
1 3
1 1

1 1 0 1 INV
2 1 1 3 4 AND
1 1 4 5 INV
2 1 2 5 6 AND";
    let output = make_bristol_array(circuit);
    //(["1 1 0 1 INV", "2 1 1 3 4 AND", "1 1 4 5 INV", "2 1 2 5 6 AND"], 7, 3, 1)

    let mut arr = output.0.clone();
    let mut wire_settings = HashMap::<String, Vec<String>>::new();
    let mut wire_hashes = HashMap::<String, Vec<String>>::new();
    let mut operations_array: Vec<Vec<String>> = Vec::new();

    set_operations_array(
        &mut arr,
        &mut wire_settings,
        &mut wire_hashes,
        &mut operations_array,
    );

    let mut initial_commitment_preimages: Vec<Vec<String>> = Vec::new();
    let mut initial_commitment_hashes: Vec<(String, String)> = Vec::<(String, String)>::new();
    generate_bit_commitments(
        &wire_settings,
        &mut initial_commitment_preimages,
        &mut initial_commitment_hashes,
    );

    let mut subsequent_commitment_preimages: Vec<Vec<String>> = Vec::new();
    let mut subsequent_commitment_hashes: Vec<(String, String)> = Vec::<(String, String)>::new();
    generate_bit_subsequent_commitments(
        &operations_array,
        &mut subsequent_commitment_preimages,
        &mut subsequent_commitment_hashes,
    );

    let secp = Secp256k1::new();
    let sk_prover = SecretKey::new(&mut rand::thread_rng());
    let prover = Keypair::from_secret_key(&secp, &sk_prover);
    let sk_verifier = SecretKey::new(&mut rand::thread_rng());
    let verifier = Keypair::from_secret_key(&secp, &sk_verifier);

    let verification_address = generate_verification_address(
        prover.x_only_public_key().0,
        verifier.x_only_public_key().0,
        Network::Testnet,
        100,
        operations_array,
    );

    let bit_commitment_address = generate_bit_commitment_address(
        prover.x_only_public_key().0,
        verifier.x_only_public_key().0,
        Network::Testnet,
        50,
        initial_commitment_hashes,
        subsequent_commitment_hashes,
    );

    println!("verification_address {}", verification_address);
    println!("bit_commitment_address {:?}", bit_commitment_address);
}
