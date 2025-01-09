# Info
Extract scriptpubkeys statistics per block using `blocks_iterator` on the Bitcoin blockchain.

This project works analizing the block files from a Bitcoin core fullnode and create a file with al the statistics on scriptpubkey usage.
 
# How to use
You will need to install `blocks_iterator_cli`.

```
cargo install blocks_iterator_cli
```

After that you can intall this project.

```
cargo install --path .
```

Then finally you can launch the tools using pipe.

```
blocks_iterator_cli --blocks-dir .bitcoin/blocks/ --network bitcoin | scriptpubkeys_per_block
```

# Results 
The result is a file called `scriptpubkeys_per_block.csv` with one line for each block and the following structure.

|Height|Timestamp |Nonce     |Tx|Empty|P2PK|P2PKH|P2SH|BareMultisig|P2WSH|P2WPKH|P2TR|OpReturn|Others|
|------|----------|----------|--|-----|----|-----|----|------------|-----|------|----|--------|------|
|0     |1231006505|2083236893|1 |0    |1   |0    |0   |0           |0    |0     |0   |0       |0     |
|1     |1231469665|2573394689|1 |0    |1   |0    |0   |0           |0    |0     |0   |0       |0     |
|2     |1231469744|1639830024|1 |0    |1   |0    |0   |0           |0    |0     |0   |0       |0     |

Where:

- Height, the height of the block
- Timestamp, the timestamp of the block
- Nonce, the nonce of the block
- Tx, number of transaction included in block (including coinbase) 
- Empty, number of outputs with empty scriptpubkey
- P2PK, number of outputs with P2PK scriptpubkey 
- P2PKH, number of outputs with P2PKH scriptpubkey
- P2SH, number of outputs with P2SH scriptpubkey
- BareMultisig, number of outputs with Multisig scriptpubkey (Bare multisig)
- P2WSH, number of outputs with P2WSH scriptpubkey (segwit)
- P2WPKH, number of outputs with P2WPKH scriptpubkey (segwit)
- P2TR, number of outputs with taproot scriptpubkey
- OpReturn, number of outputs with OpReturn
- Others, number of outputs with script different from previous classes
