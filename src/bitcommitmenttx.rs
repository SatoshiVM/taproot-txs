use crate::timelock::create_extra_leaf;

use bitcoin::opcodes::all::{
    OP_BOOLOR, OP_CHECKSIG, OP_CHECKSIGADD, OP_CSV, OP_DROP, OP_EQUAL, OP_SHA256, OP_SWAP,
    OP_VERIFY,
};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::taproot::TaprootBuilder;
use bitcoin::{script, secp256k1, Address, Network, ScriptBuf};

pub fn generate_bit_commitment_address(
    pubkey: secp256k1::XOnlyPublicKey,
    verifier_key: secp256k1::XOnlyPublicKey,
    network: Network,
    time: u32,
    initial_commitment_hashes: Vec<(String, String)>,
    subsequent_commitment_hashes: Vec<(String, String)>,
) -> Address {
    //let pubkey = XOnlyPublicKey::from_str(pubkey).unwrap();
    //let vickys_key = XOnlyPublicKey::from_str(vickys_key).unwrap();

    let leaf1 = script::Builder::new()
        .push_int(10 as i64)
        .push_opcode(OP_CSV)
        .push_opcode(OP_DROP)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();

    let leaf2 = script::Builder::new()
        .push_int(0 as i64)
        .push_x_only_key(&pubkey)
        .push_opcode(OP_CHECKSIGADD)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIGADD)
        .push_int(2 as i64)
        .push_opcode(OP_EQUAL)
        .into_script();

    let mut bit_commitment_script = Vec::new();
    for hash_pair in initial_commitment_hashes
        .iter()
        .chain(&subsequent_commitment_hashes)
    {
        let mut first = [0u8; 32];
        first.copy_from_slice(&hex::decode(&hash_pair.0.clone()).unwrap());
        let mut second = [0u8; 32];
        second.copy_from_slice(&hex::decode(&hash_pair.1.clone()).unwrap());
        bit_commitment_script.extend(template_script_hex(first, second));
    }
    bit_commitment_script.extend(template_last_script(pubkey));
    let leaf3 = ScriptBuf::from_bytes(bit_commitment_script.clone());

    let leaf4 = create_extra_leaf(time, pubkey);

    let secp = Secp256k1::new();

    let taproot_spend_info = TaprootBuilder::new()
        .add_leaf(2, leaf1.clone())
        .expect("adding leaf should work")
        .add_leaf(2, leaf2.clone())
        .expect("adding leaf should work")
        .add_leaf(2, leaf3.clone())
        .expect("adding leaf should work")
        .add_leaf(2, leaf4.clone())
        .expect("adding leaf should work")
        .finalize(&secp, pubkey)
        .expect("finalizing taproot builder should work");

    let address = Address::p2tr(&secp, pubkey, taproot_spend_info.merkle_root(), network);
    address
}

pub fn template_script_hex(first_input: [u8; 32], second_input: [u8; 32]) -> Vec<u8> {
    let builder = script::Builder::new();
    builder
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUAL)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUAL)
        .push_opcode(OP_BOOLOR)
        .push_opcode(OP_VERIFY)
        .into_bytes()
}

pub fn template_last_script(xonlybubkey: secp256k1::XOnlyPublicKey) -> Vec<u8> {
    let builder = script::Builder::new();
    builder
        .push_x_only_key(&xonlybubkey)
        .push_opcode(OP_CHECKSIG)
        .into_bytes()
}

#[test]
fn test_bit_commitment_tx() {
    let circuit = "4 7
1 3
1 1

1 1 0 1 INV
2 1 1 3 4 AND
1 1 4 5 INV
2 1 2 5 6 AND";
    let output = make_bristol_array(circuit);
    println!("{:?}", output);
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
    println!("arr {:?}", arr);
    println!("wire_settings {:?}", wire_settings);
    println!("wire_hashes {:?}", wire_hashes);
    println!("operations_array {:?}", operations_array);

    let mut initial_commitment_preimages: Vec<Vec<String>> = Vec::new();
    let mut initial_commitment_hashes: Vec<(String, String)> = Vec::<(String, String)>::new();
    generate_bit_commitments(
        &wire_settings,
        &mut initial_commitment_preimages,
        &mut initial_commitment_hashes,
    );
    println!(
        "initial_commitment_preimages {:?}",
        initial_commitment_preimages
    );
    println!("initial_commitment_hashes {:?}", initial_commitment_hashes);
    let mut subsequent_commitment_preimages: Vec<Vec<String>> = Vec::new();
    let mut subsequent_commitment_hashes: Vec<(String, String)> = Vec::<(String, String)>::new();
    generate_bit_subsequent_commitments(
        &operations_array,
        &mut subsequent_commitment_preimages,
        &mut subsequent_commitment_hashes,
    );
    println!(
        "subsequent_commitment_preimages {:?}",
        subsequent_commitment_preimages
    );
    println!(
        "subsequent_commitment_hashes {:?}",
        subsequent_commitment_hashes
    );

    let secp = Secp256k1::new();
    let sk_prover = SecretKey::new(&mut rand::thread_rng());
    let prover = Keypair::from_secret_key(&secp, &sk_prover);
    let sk_verifier = SecretKey::new(&mut rand::thread_rng());
    let verifier = Keypair::from_secret_key(&secp, &sk_verifier);
    let bit_commitment_address = generate_bit_commitment_address(
        prover.x_only_public_key().0,
        verifier.x_only_public_key().0,
        Network::Testnet,
        50,
        initial_commitment_hashes,
        subsequent_commitment_hashes,
    );
    println!("bit_commitment_address {:?}", bit_commitment_address);
}
