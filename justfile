# Devnet integration tests

build:
    anchor build -- --features devnet

deploy:
    anchor deploy -p vrf-demo --provider.cluster d --program-keypair target/deploy/vrf_demo-keypair.json --provider.wallet dev-wallet.json --no-idl
