# VRF Demo

Anchor program showing how to request and consume verifiable randomness from common providers.

[Source Repository](https://github.com/ChiefWoods/vrf-demo)

## How It Works

Each instruction can be invoked to generate entropic bytes from different randomness providers. The following providers are supported:

- Switchboard On Demand
- MagicBlock
- Blueshift ECVRF

Note: Orao not added yet, pending Anchor v1 bump

## Built With

### Languages

- [![Anchor](https://img.shields.io/badge/Anchor-0e0d11?style=for-the-badge)](https://anchor-lang.com/)

## Getting Started

### Prerequisites

1. Update your Solana CLI

```sh
agave-install init 3.1.10
```

2. Update your Anchor

```sh
avm use 1.2.0
```

### Setup

1. Clone the repository

```sh
git clone https://github.com/ChiefWoods/vrf-demo.git
```

2. Resync your program id

```sh
cd programs/vrf-demo
anchor keys sync
```

3. Build the program

```sh
just build
```

#### Testing

Run tests.

```sh
just test pyth-core
```

## Issues

View the [open issues](https://github.com/ChiefWoods/vrf-demo/issues) for a full list of proposed features and known bugs.

## Acknowledgements

### Resources

- [Shields.io](https://shields.io/)

## Contact

[chii.yuen@hotmail.com](mailto:chii.yuen@hotmail.com)
