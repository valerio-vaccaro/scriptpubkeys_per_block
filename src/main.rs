use blocks_iterator::PipeIterator;
use env_logger::Env;
use log::info;
use std::error::Error;
use std::io;

struct Counters {
    txno: u32,
    empty: u32,
    p2pk: u32,
    p2pkh: u32,
    p2sh: u32,
    multisig: u32,
    p2wsh: u32,
    p2wpkh: u32,
    p2tr: u32,
    opreturn: u32,
    others: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("start");
    let iter = PipeIterator::new(io::stdin(), None);

    println!("Height, Timestamp, Nonce, Tx, Empty, P2PK, P2PKH, P2SH, BareMultisig, P2WSH, P2WPKH, P2TR, OpReturn, Others");

    for block_extra in iter {
        // for each block
        let mut counters = Counters {
            txno: 0,
            empty: 0,
            p2pk: 0,
            p2pkh: 0,
            p2sh: 0,
            multisig: 0,
            p2wsh: 0,
            p2wpkh: 0,
            p2tr: 0,
            opreturn: 0,
            others: 0,
        };

        for (_txid, tx) in block_extra.iter_tx() {
            // for each transaction
            counters.txno += 1;

            for (_i, output) in tx.output.iter().enumerate() {
                // for each output
                if output.script_pubkey.is_empty() {
                    counters.empty += 1;
                } else if output.script_pubkey.is_p2pk() {
                    counters.p2pk += 1;
                } else if output.script_pubkey.is_p2pkh() {
                    counters.p2pkh += 1;
                } else if output.script_pubkey.is_p2sh() {
                    counters.p2sh += 1;
                } else if output.script_pubkey.is_multisig() {
                    counters.multisig += 1;
                } else if output.script_pubkey.is_p2wsh() {
                    counters.p2wsh += 1;
                } else if output.script_pubkey.is_p2wpkh() {
                    counters.p2wpkh += 1;
                } else if output.script_pubkey.is_p2tr() {
                    counters.p2tr += 1;
                } else if output.script_pubkey.is_op_return() {
                    counters.opreturn += 1;
                } else {
                    counters.others += 1;
                }
            } // output
        } // transaction

        println!(
            "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            block_extra.height(),
            block_extra.block().header.time,
            block_extra.block().header.nonce,
            counters.txno,
            counters.empty,
            counters.p2pk,
            counters.p2pkh,
            counters.p2sh,
            counters.multisig,
            counters.p2wsh,
            counters.p2wpkh,
            counters.p2tr,
            counters.opreturn,
            counters.others
        );
    } // block
    info!("stop");
    Ok(())
}
