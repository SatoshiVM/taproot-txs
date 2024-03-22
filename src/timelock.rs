use bitcoin::opcodes::all::{OP_CHECKSIG, OP_CLTV, OP_DROP};
use bitcoin::{script, secp256k1, ScriptBuf};

pub fn create_extra_leaf(time: u32, pubkey: secp256k1::XOnlyPublicKey) -> ScriptBuf {
    let time_lock_script = script::Builder::new()
        .push_int(time as i64)
        .push_opcode(OP_CLTV)
        .push_opcode(OP_DROP)
        .push_x_only_key(&pubkey)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    time_lock_script
}
