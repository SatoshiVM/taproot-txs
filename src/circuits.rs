use crate::utils::{get_rand, hex_to_bytes, sha256};
use std::collections::HashMap;

pub fn make_bristol_array(arrprep: &str) -> (Vec<String>, u32, u32, u32) {
    let mut arr: Vec<String> = arrprep.split('\n').map(|s| s.trim().to_string()).collect();

    arr = arr
        .iter()
        .map(|entry| {
            if entry.starts_with(" ") {
                entry[1..].to_string()
            } else {
                entry.clone()
            }
        })
        .collect();

    if arr[0].is_empty() {
        arr.remove(0);
    }
    if arr.last().map_or(false, |s| s.is_empty()) {
        arr.pop();
    }
    if arr.get(3).is_none() {
        // to add error handling
        println!("Oops, you entered an invalid bristol circuit! Try again with the whole document, including the first three lines that define the number of gates, number of input bits, and number of output bits.");
    }
    let number_of_preimages_to_expect = arr[0]
        .split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse::<u32>()
        .unwrap_or(0);
    let number_of_inputs = arr[1]
        .split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse::<u32>()
        .unwrap_or(0);
    let number_of_outputs = arr[2]
        .split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse::<u32>()
        .unwrap_or(0);

    arr.drain(0..4);

    (
        arr,
        number_of_preimages_to_expect,
        number_of_inputs,
        number_of_outputs,
    )
}

pub fn set_operations_array(
    arr: &mut Vec<String>,
    wire_settings: &mut HashMap<String, Vec<String>>,
    wire_hashes: &mut HashMap<String, Vec<String>>,
    operations_array: &mut Vec<Vec<String>>,
) {
    for index in 0..arr.len() {
        let gate: Vec<String> = arr[index]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        match gate.last().unwrap_or(&"".to_string()).as_str() {
            "INV" => {
                let key = gate[2].clone();

                let input_preimages = wire_settings
                    .entry(key.clone())
                    .or_insert_with(|| vec![get_rand(32), get_rand(32)])
                    .clone();

                let output_preimages = vec![get_rand(32), get_rand(32)];

                let input_hashes = vec![
                    sha256(&hex_to_bytes(&input_preimages[0].clone())),
                    sha256(&hex_to_bytes(&input_preimages[1].clone())),
                ];

                wire_hashes.insert(key, input_hashes.clone());

                let output_hashes = vec![
                    sha256(&hex_to_bytes(&output_preimages[0].clone())),
                    sha256(&hex_to_bytes(&output_preimages[1].clone())),
                ];

                wire_settings.insert(gate[3].clone(), output_preimages.clone());
                wire_hashes.insert(gate[3].clone(), output_hashes.clone());

                operations_array.push(vec![
                    "INV".to_string(),
                    format!(
                        "input_preimages {} {}",
                        input_preimages[0], input_preimages[1]
                    ),
                    format!(
                        "output_preimages {} {}",
                        output_preimages[0], output_preimages[1]
                    ),
                    format!("input_hashes {} {}", input_hashes[0], input_hashes[1]),
                    format!("output_hashes {} {}", output_hashes[0], output_hashes[1]),
                    format!("var w_{} = INV( wires[ {} ] )", gate[3], gate[2]),
                ]);
            }

            "AND" => {
                let first_key = gate[2].clone();
                let second_key = gate[3].clone();

                let first_input_preimages = wire_settings
                    .entry(first_key.clone())
                    .or_insert_with(|| vec![get_rand(32), get_rand(32)])
                    .clone();

                let second_input_preimages = wire_settings
                    .entry(second_key.clone())
                    .or_insert_with(|| vec![get_rand(32), get_rand(32)])
                    .clone();

                let first_input_hashes = vec![
                    sha256(&hex_to_bytes(&first_input_preimages[0].clone())),
                    sha256(&hex_to_bytes(&first_input_preimages[1].clone())),
                ];
                let second_input_hashes = vec![
                    sha256(&hex_to_bytes(&second_input_preimages[0].clone())),
                    sha256(&hex_to_bytes(&second_input_preimages[1].clone())),
                ];

                let output_preimages = vec![get_rand(32), get_rand(32)];
                let output_hashes = vec![
                    sha256(&hex_to_bytes(&output_preimages[0].clone())),
                    sha256(&hex_to_bytes(&output_preimages[1].clone())),
                ];

                wire_hashes.insert(first_key, first_input_hashes.clone());
                wire_hashes.insert(second_key, second_input_hashes.clone());
                wire_settings.insert(gate[4].clone(), output_preimages.clone());
                wire_hashes.insert(gate[4].clone(), output_hashes.clone());

                operations_array.push(vec![
                    "AND".to_string(),
                    format!(
                        "first_input_preimages {} {}",
                        first_input_preimages[0], first_input_preimages[1]
                    ),
                    format!(
                        "second_input_preimages {} {}",
                        second_input_preimages[0], second_input_preimages[1]
                    ),
                    format!(
                        "output_preimages {} {}",
                        output_preimages[0], output_preimages[1]
                    ),
                    format!(
                        "first_input_hashes {} {}",
                        first_input_hashes[0], first_input_hashes[1]
                    ),
                    format!(
                        "second_input_hashes {} {}",
                        second_input_hashes[0], second_input_hashes[1]
                    ),
                    format!("output_hashes {} {}", output_hashes[0], output_hashes[1]),
                    format!(
                        "var w_{} = AND( wires[ {} ], wires[ {} ] )",
                        gate[4], gate[2], gate[3]
                    ),
                ]);
            }

            "XOR" => {
                let (first_key, second_key, output_key) =
                    (gate[2].clone(), gate[3].clone(), gate[4].clone());

                let first_input_preimages = wire_settings
                    .entry(first_key.clone())
                    .or_insert_with(|| vec![get_rand(32), get_rand(32)])
                    .clone();

                let second_input_preimages = wire_settings
                    .entry(second_key.clone())
                    .or_insert_with(|| vec![get_rand(32), get_rand(32)])
                    .clone();

                let first_input_hashes = vec![
                    sha256(&hex_to_bytes(&first_input_preimages[0].clone())),
                    sha256(&hex_to_bytes(&first_input_preimages[1].clone())),
                ];
                let second_input_hashes = vec![
                    sha256(&hex_to_bytes(&second_input_preimages[0].clone())),
                    sha256(&hex_to_bytes(&second_input_preimages[1].clone())),
                ];

                let output_preimages = vec![get_rand(32), get_rand(32)];
                let output_hashes = vec![
                    sha256(&hex_to_bytes(&output_preimages[0].clone())),
                    sha256(&hex_to_bytes(&output_preimages[1].clone())),
                ];

                wire_hashes.insert(first_key, first_input_hashes.clone());
                wire_hashes.insert(second_key, second_input_hashes.clone());
                wire_settings.insert(output_key.clone(), output_preimages.clone());
                wire_hashes.insert(output_key, output_hashes.clone());

                operations_array.push(vec![
                    "XOR".to_string(),
                    format!(
                        "first_input_preimages {} {}",
                        first_input_preimages[0], first_input_preimages[1]
                    ),
                    format!(
                        "second_input_preimages {} {}",
                        second_input_preimages[0], second_input_preimages[1]
                    ),
                    format!(
                        "output_preimages {} {}",
                        output_preimages[0], output_preimages[1]
                    ),
                    format!(
                        "first_input_hashes {} {}",
                        first_input_hashes[0], first_input_hashes[1]
                    ),
                    format!(
                        "second_input_hashes {} {}",
                        second_input_hashes[0], second_input_hashes[1]
                    ),
                    format!("output_hashes {} {}", output_hashes[0], output_hashes[1]),
                    format!(
                        "var w_{} = XOR( wires[ {} ], wires[ {} ] )",
                        gate[4], gate[2], gate[3]
                    ),
                ]);
            }

            _ => {}
        }
    }
}

pub fn generate_bit_commitments(
    wire_settings: &HashMap<String, Vec<String>>,
    initial_commitment_preimages: &mut Vec<Vec<String>>,
    initial_commitment_hashes: &mut Vec<(String, String)>,
    input_number: Vec<u8>,
) {
    for i in input_number {
        let key = i.to_string();
        if let Some(preimages) = wire_settings.get(&key) {
            initial_commitment_preimages.push(preimages.clone());

            let hash_0 = sha256(&hex_to_bytes(&preimages[0].clone()));
            let hash_1 = sha256(&hex_to_bytes(&preimages[1].clone()));
            initial_commitment_hashes.push((hash_0, hash_1));
        }
    }
}

pub fn generate_bit_subsequent_commitments(
    operations_array: &Vec<Vec<String>>,
    subsequent_commitment_preimages: &mut Vec<Vec<String>>,
    subsequent_commitment_hashes: &mut Vec<(String, String)>,
) {
    operations_array.iter().for_each(|operation| {
        if operation[0] == "INV" {
            subsequent_commitment_preimages.push(vec![
                choice_index_space(&operation[2], 1),
                choice_index_space(&operation[2], 2),
            ])
        };
        if operation[0] == "INV" {
            subsequent_commitment_hashes.push((
                choice_index_space(&operation[4], 1),
                choice_index_space(&operation[4], 2),
            ))
        };
        if operation[0] == "AND" {
            subsequent_commitment_preimages.push(vec![
                choice_index_space(&operation[3], 1),
                choice_index_space(&operation[3], 2),
            ])
        };
        if operation[0] == "AND" {
            subsequent_commitment_hashes.push((
                choice_index_space(&operation[6], 1),
                choice_index_space(&operation[6], 2),
            ))
        };
        if operation[0] == "XOR" {
            subsequent_commitment_preimages.push(vec![
                choice_index_space(&operation[3], 1),
                choice_index_space(&operation[3], 2),
            ])
        };
        if operation[0] == "XOR" {
            subsequent_commitment_hashes.push((
                choice_index_space(&operation[6], 1),
                choice_index_space(&operation[6], 2),
            ))
        };
    });
}

pub fn choice_index_space(input: &String, index: usize) -> String {
    let all: Vec<&str> = input.as_str().split_whitespace().collect();
    let str = all[index].clone();
    str.to_string()
}
