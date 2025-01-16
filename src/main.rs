use blocks_iterator::PipeIterator;
use env_logger::Env;
use log::info;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("start");
    let iter = PipeIterator::new(io::stdin(), None);

    println!("Height, Timestamp, Nonce, Tx, Empty, P2PK, P2PKH, P2SH, BareMultisig, P2WSH, P2WPKH, P2TR, OpReturn, Others");

    for block_extra in iter {
        // for each block
        let mut txno = 0;
        let mut empty = 0;
        let mut p2pk = 0;
        let mut p2pkh = 0;
        let mut p2sh = 0;
        let mut multisig = 0;
        let mut p2wsh = 0;
        let mut p2wpkh = 0;
        let mut p2tr = 0;
        let mut opreturn = 0;
        let mut others = 0;

        for (_txid, tx) in block_extra.iter_tx() {
            // for each transaction
            txno = txno + 1;

            for (_i, output) in tx.output.iter().enumerate() {
                // for each output
                if output.script_pubkey.is_empty() {
                    empty = empty + 1;
                } else if output.script_pubkey.is_p2pk() {
                    p2pk = p2pk + 1;
                } else if output.script_pubkey.is_p2pkh() {
                    p2pkh = p2pkh + 1;
                } else if output.script_pubkey.is_p2sh() {
                    p2sh = p2sh + 1;
                } else if output.script_pubkey.is_multisig() {
                    multisig = multisig + 1;
                } else if output.script_pubkey.is_p2wsh() {
                    p2wsh = p2wsh + 1;
                } else if output.script_pubkey.is_p2wpkh() {
                    p2wpkh = p2wpkh + 1;
                } else if output.script_pubkey.is_p2tr() {
                    p2tr = p2tr + 1;
                } else if output.script_pubkey.is_op_return() {
                    opreturn = opreturn + 1;
                } else {
                    others = others + 1;
                }
            } // output
        } // transaction

        println!(
            "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            block_extra.height(),
            block_extra.block().header.time,
            block_extra.block().header.nonce,
            txno,
            empty,
            p2pk,
            p2pkh,
            p2sh,
            multisig,
            p2wsh,
            p2wpkh,
            p2tr,
            opreturn,
            others
        );
    } // block
    info!("stop");
    Ok(())
}
