
cargo +nightly contract build --release

# Deploy the contract with initial parameters (supply=1000, price=450)
cargo contract instantiate --constructor new --args 1000 450
