use crate::circuits::choice_index_space;
use crate::gate_templates::*;
use crate::timelock::create_extra_leaf;

use bitcoin::secp256k1::Secp256k1;
use bitcoin::taproot::TaprootBuilder;
use bitcoin::{secp256k1, Address, Network};

pub fn generate_verification_address(
    pubkey: secp256k1::XOnlyPublicKey,
    verifier_key: secp256k1::XOnlyPublicKey,
    network: Network,
    time: u32,
    operations_array: Vec<Vec<String>>,
) -> Address {
    let mut challenge_scripts = Vec::new();

    operations_array.iter().for_each(|operation| {
        if operation[0] == "INV" {
            let input_hash_pair = vec![
                choice_index_space(&operation[3], 1),
                choice_index_space(&operation[3], 2),
            ];
            let output_hash_pair = vec![
                choice_index_space(&operation[4], 1),
                choice_index_space(&operation[4], 2),
            ];
            let mut first = [0u8; 32];
            first.copy_from_slice(&hex::decode(&input_hash_pair[0]).unwrap());
            let mut second = [0u8; 32];
            second.copy_from_slice(&hex::decode(&input_hash_pair[1]).unwrap());
            let input_hash_pair = vec![first, second];

            let mut first = [0u8; 32];
            first.copy_from_slice(&hex::decode(&output_hash_pair[0]).unwrap());
            let mut second = [0u8; 32];
            second.copy_from_slice(&hex::decode(&output_hash_pair[1]).unwrap());
            let output_hash_pair = vec![first, second];

            challenge_scripts.push((
                1,
                template_op_not_00(
                    input_hash_pair[0].clone(),
                    output_hash_pair[0].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_not_01(
                    input_hash_pair[0].clone(),
                    output_hash_pair[1].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_not_10(
                    input_hash_pair[1].clone(),
                    output_hash_pair[0].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_not_11(
                    input_hash_pair[1].clone(),
                    output_hash_pair[1].clone(),
                    verifier_key,
                ),
            ));
        }

        if operation[0] == "AND" {
            let input_hash_pair = vec![
                choice_index_space(&operation[4], 1),
                choice_index_space(&operation[4], 2),
            ];
            let input_hash_pair_2 = vec![
                choice_index_space(&operation[5], 1),
                choice_index_space(&operation[5], 2),
            ];
            let output_hash_pair = vec![
                choice_index_space(&operation[6], 1),
                choice_index_space(&operation[6], 2),
            ];
            let mut first = [0u8; 32];
            first.copy_from_slice(&hex::decode(&input_hash_pair[0]).unwrap());
            let mut second = [0u8; 32];
            second.copy_from_slice(&hex::decode(&input_hash_pair[1]).unwrap());
            let input_hash_pair = vec![first, second];

            let mut first = [0u8; 32];
            first.copy_from_slice(&hex::decode(&input_hash_pair_2[0]).unwrap());
            let mut second = [0u8; 32];
            second.copy_from_slice(&hex::decode(&input_hash_pair_2[1]).unwrap());
            let input_hash_pair_2 = vec![first, second];

            let mut first = [0u8; 32];
            first.copy_from_slice(&hex::decode(&output_hash_pair[0]).unwrap());
            let mut second = [0u8; 32];
            second.copy_from_slice(&hex::decode(&output_hash_pair[1]).unwrap());
            let output_hash_pair = vec![first, second];

            challenge_scripts.push((
                1,
                template_op_booland_000(
                    input_hash_pair[0].clone(),
                    input_hash_pair_2[0].clone(),
                    output_hash_pair[0].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_booland_001(
                    input_hash_pair[0].clone(),
                    input_hash_pair_2[0].clone(),
                    output_hash_pair[1].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_booland_010(
                    input_hash_pair[0].clone(),
                    input_hash_pair_2[1].clone(),
                    output_hash_pair[0].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_booland_011(
                    input_hash_pair[0].clone(),
                    input_hash_pair_2[1].clone(),
                    output_hash_pair[1].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_booland_100(
                    input_hash_pair[1].clone(),
                    input_hash_pair_2[0].clone(),
                    output_hash_pair[0].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_booland_101(
                    input_hash_pair[1].clone(),
                    input_hash_pair_2[0].clone(),
                    output_hash_pair[1].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_booland_110(
                    input_hash_pair[1].clone(),
                    input_hash_pair_2[1].clone(),
                    output_hash_pair[0].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_booland_111(
                    input_hash_pair[1].clone(),
                    input_hash_pair_2[1].clone(),
                    output_hash_pair[1].clone(),
                    verifier_key,
                ),
            ));
        }

        if operation[0] == "XOR" {
            let input_hash_pair = vec![
                choice_index_space(&operation[4], 1),
                choice_index_space(&operation[4], 2),
            ];
            let input_hash_pair_2 = vec![
                choice_index_space(&operation[5], 1),
                choice_index_space(&operation[5], 2),
            ];
            let output_hash_pair = vec![
                choice_index_space(&operation[6], 1),
                choice_index_space(&operation[6], 2),
            ];
            let mut first = [0u8; 32];
            first.copy_from_slice(&hex::decode(&input_hash_pair[0]).unwrap());
            let mut second = [0u8; 32];
            second.copy_from_slice(&hex::decode(&input_hash_pair[1]).unwrap());
            let input_hash_pair = vec![first, second];

            let mut first = [0u8; 32];
            first.copy_from_slice(&hex::decode(&input_hash_pair_2[0]).unwrap());
            let mut second = [0u8; 32];
            second.copy_from_slice(&hex::decode(&input_hash_pair_2[1]).unwrap());
            let input_hash_pair_2 = vec![first, second];

            let mut first = [0u8; 32];
            first.copy_from_slice(&hex::decode(&output_hash_pair[0]).unwrap());
            let mut second = [0u8; 32];
            second.copy_from_slice(&hex::decode(&output_hash_pair[1]).unwrap());
            let output_hash_pair = vec![first, second];

            challenge_scripts.push((
                1,
                template_op_xor_000(
                    input_hash_pair[0].clone(),
                    input_hash_pair_2[0].clone(),
                    output_hash_pair[0].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_xor_001(
                    input_hash_pair[0].clone(),
                    input_hash_pair_2[0].clone(),
                    output_hash_pair[1].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_xor_010(
                    input_hash_pair[0].clone(),
                    input_hash_pair_2[1].clone(),
                    output_hash_pair[0].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_xor_011(
                    input_hash_pair[0].clone(),
                    input_hash_pair_2[1].clone(),
                    output_hash_pair[1].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_xor_100(
                    input_hash_pair[1].clone(),
                    input_hash_pair_2[0].clone(),
                    output_hash_pair[0].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_xor_101(
                    input_hash_pair[1].clone(),
                    input_hash_pair_2[0].clone(),
                    output_hash_pair[1].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_xor_110(
                    input_hash_pair[1].clone(),
                    input_hash_pair_2[1].clone(),
                    output_hash_pair[0].clone(),
                    verifier_key,
                ),
            ));
            challenge_scripts.push((
                1,
                template_op_xor_111(
                    input_hash_pair[1].clone(),
                    input_hash_pair_2[1].clone(),
                    output_hash_pair[1].clone(),
                    verifier_key,
                ),
            ));
        }
    });

    challenge_scripts.push((1, create_extra_leaf(time, pubkey)));

    let secp = Secp256k1::new();
    let tree = TaprootBuilder::with_huffman_tree(challenge_scripts).unwrap();
    let spend_info = tree.finalize(&secp, pubkey).unwrap();
    let address = Address::p2tr(&secp, pubkey, spend_info.merkle_root(), network);
    address
}

#[test]
fn test_verification_tx() {
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

    let address = generate_verification_address(
        prover.x_only_public_key().0,
        verifier.x_only_public_key().0,
        Network::Testnet,
        100,
        operations_array,
    );
    println!("address {}", address);
}

#[test]
fn test_build_tree() {
    let secp = Secp256k1::new();
    let sk = SecretKey::new(&mut rand::thread_rng());

    let keypair = Keypair::from_secret_key(&secp, &sk);
    let (internal_key, _) = keypair.x_only_public_key();

    let script = script::Builder::new()
        .push_opcode(OP_CHECKSIG)
        .into_script();

    // huffman tree
    let tree = TaprootBuilder::with_huffman_tree(vec![
        (1, script.clone()),
        (1, script.clone()),
        (1, script.clone()),
    ])
    .unwrap();
    let spend_info_1 = tree.finalize(&secp, internal_key).unwrap();

    // add leaf
    let mut taproot = TaprootBuilder::new();
    for _ in 0..2 {
        taproot = taproot.add_leaf(2, script.clone()).unwrap();
    }
    taproot = taproot.add_leaf(1, script.clone()).unwrap();
    let spend_info_2 = taproot.finalize(&secp, internal_key).unwrap();

    assert_eq!(spend_info_1.merkle_root(), spend_info_2.merkle_root());
}
