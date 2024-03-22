use bitcoin::opcodes::all::{
    OP_BOOLAND, OP_CHECKSIG, OP_EQUALVERIFY, OP_FROMALTSTACK, OP_NOT, OP_NUMNOTEQUAL, OP_SHA256,
    OP_SWAP, OP_TOALTSTACK, OP_VERIFY,
};
use bitcoin::opcodes::OP_0;
use bitcoin::{script, secp256k1, ScriptBuf};

pub fn template_op_not_00(
    first_input: [u8; 32],
    second_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_0)
        .push_opcode(OP_NOT)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_0)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_not_01(
    first_input: [u8; 32],
    second_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_0)
        .push_opcode(OP_NOT)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_not_10(
    first_input: [u8; 32],
    second_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NOT)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_not_11(
    first_input: [u8; 32],
    second_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NOT)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_booland_000(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_BOOLAND)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_booland_001(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_BOOLAND)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_booland_010(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_BOOLAND)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_booland_011(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_BOOLAND)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_booland_100(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_BOOLAND)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_booland_101(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_BOOLAND)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_booland_110(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_BOOLAND)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_booland_111(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_BOOLAND)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_xor_000(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_xor_001(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_xor_010(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_xor_011(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_xor_100(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_xor_101(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_xor_110(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(0i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}

pub fn template_op_xor_111(
    first_input: [u8; 32],
    second_input: [u8; 32],
    third_input: [u8; 32],
    verifier_key: secp256k1::XOnlyPublicKey,
) -> ScriptBuf {
    let leaf1 = script::Builder::new()
        .push_opcode(OP_TOALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(first_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_SWAP)
        .push_opcode(OP_SHA256)
        .push_slice(second_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_FROMALTSTACK)
        .push_opcode(OP_SHA256)
        .push_slice(third_input)
        .push_opcode(OP_EQUALVERIFY)
        .push_int(1i64)
        .push_opcode(OP_NUMNOTEQUAL)
        .push_opcode(OP_VERIFY)
        .push_x_only_key(&verifier_key)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    leaf1
}
