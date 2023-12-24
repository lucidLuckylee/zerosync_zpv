# ZeroSync Zipped Proof Verifier

Verify a [ZeroSync](zerosync.org/demo) header chain proof from a zip file.
Usually used in conjunction with [Blockstream satellite](https://blockstream.github.io/satellite/).

## Usage

```sh
# Verify an archived proof
cargo run -r proof_data zerosync_proof.tar.gz

# Run with a receiver for blockstream satellite
blocksat-cli blocksat-cli api demo-rx

# If the archive is encrypted with your public key

# WARNING: High security risk!
# If security is no issue for you you can handle every received message as a potential proof and validate it.
# For this you have to comment out lines 198-199 in api/api.py of your blocksat-cli python installation.
blocksat-cli api listen --demo --exec 'cargo run -r proof_data {}' --insecure --plaintext --save-raw

# Without automatic exec: Receive the proof and verify it manually
blocksat-cli api listen --plaintext --save-raw
cargo run -r proof_data ~/.blocksat/api/downloads/RECEIVED_FILE
```


