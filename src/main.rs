use bitcoin_slices::bitcoin;
use bitcoin_slices::bitcoin::Script;
use bitcoin_slices::bsl;
use bitcoin_slices::Visit;
use bitcoin_slices::Visitor;
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

    fn update_from_script(&mut self, script_pubkey: &Script) {
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
        let mut visitor = CountersVisitor::default();

        bsl::Block::visit(block_extra.block_bytes(), &mut visitor).expect("block bytes is a block");

        println!(
            "{}, {}, {}, {}",
            block_extra.height(),
            block_extra.block().header.time,
            block_extra.block().header.nonce,
            visitor.counters
        );
    } // block
    info!("stop");
    Ok(())
}

#[derive(Default)]
struct CountersVisitor {
    counters: Counters,
}

impl Visitor for CountersVisitor {
    fn visit_transaction(&mut self, _tx: &bsl::Transaction) -> core::ops::ControlFlow<()> {
        self.counters.increment_txno();
        core::ops::ControlFlow::Continue(())
    }
    fn visit_tx_out(&mut self, _vout: usize, tx_out: &bsl::TxOut) -> core::ops::ControlFlow<()> {
        let s = bitcoin::Script::from_bytes(tx_out.script_pubkey());
        self.counters.update_from_script(&s);
        core::ops::ControlFlow::Continue(())
    }
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
