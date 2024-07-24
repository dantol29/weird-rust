## Steps to run

1. Create a new project woth `dfx new <project_name>` (choose rust, no front-end canister, no extra features)

2. Copy .rs files from this repo to src

3. Add EVM RPC canister to dfx.json

4. Copy candid(.did) file

5. Add these dependencies `serde = "1.0.193", serde_bytes = "0.11.12"` to Cargo.toml

6. Run `cargo update` to install new dependencies

7. Run `dfx deps pull` to pull EVM RPC canister

8. Start local replica `dfx start --background`

9. Initialize EVM RPC canister `dfx deps init evm_rpc --argument '(record { nodesInSubnet = 28 })'`

10. Deploy EVM RPC canister `dfx deps deploy`

11. Deploy app `dfx deploy`

12. Open backend canister via Candid interface and enjoy:)!