use blocks_iterator::bitcoin::ScriptBuf;
use blocks_iterator::PipeIterator;
use env_logger::Env;
use log::info;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Default)]
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

impl Counters {
    fn increment_txno(&mut self) {
        self.txno += 1;
    }

    fn update_from_script(&mut self, script_pubkey: &ScriptBuf) {
        if script_pubkey.is_empty() {
            self.empty += 1;
        } else if script_pubkey.is_p2pk() {
            self.p2pk += 1;
        } else if script_pubkey.is_p2pkh() {
            self.p2pkh += 1;
        } else if script_pubkey.is_p2sh() {
            self.p2sh += 1;
        } else if script_pubkey.is_multisig() {
            self.multisig += 1;
        } else if script_pubkey.is_p2wsh() {
            self.p2wsh += 1;
        } else if script_pubkey.is_p2wpkh() {
            self.p2wpkh += 1;
        } else if script_pubkey.is_p2tr() {
            self.p2tr += 1;
        } else if script_pubkey.is_op_return() {
            self.opreturn += 1;
        } else {
            self.others += 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("start");
    let iter = PipeIterator::new(io::stdin(), None);

    println!("Height, Timestamp, Nonce, Tx, Empty, P2PK, P2PKH, P2SH, BareMultisig, P2WSH, P2WPKH, P2TR, OpReturn, Others");

    for block_extra in iter {
        // for each block
        let mut counters = Counters::default();

        for (_txid, tx) in block_extra.iter_tx() {
            // for each transaction
            counters.increment_txno();

            for output in &tx.output {
                counters.update_from_script(&output.script_pubkey);
            } // output
        } // transaction

        println!(
            "{}, {}, {}, {}",
            block_extra.height(),
            block_extra.block().header.time,
            block_extra.block().header.nonce,
            counters
        );
    } // block
    info!("stop");
    Ok(())
}

impl fmt::Display for Counters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            self.txno,
            self.empty,
            self.p2pk,
            self.p2pkh,
            self.p2sh,
            self.multisig,
            self.p2wsh,
            self.p2wpkh,
            self.p2tr,
            self.opreturn,
            self.others
        )
    }
}
