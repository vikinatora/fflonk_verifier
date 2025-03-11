# FFlonk Verifier Client

This is a simple JavaScript client for interacting with the FFlonk Verifier contract deployed on Arbitrum Sepolia.

## Prerequisites

- Node.js (v14 or higher recommended)
- An Ethereum private key with Arbitrum Sepolia ETH for gas fees

## Installation

1. Clone this repository or download the files
2. Install dependencies:

```bash
npm install
```

3. Create your `.env` file from the example:

```bash
cp .env.example .env
```

4. Edit the `.env` file and add your private key:

```
PRIVATE_KEY=your_private_key_here
```

## Usage

There are multiple ways to use this script to verify proofs:

### Using Default Example Values

```bash
npm run verify
```

This will use the example proof and public input defined in the script.

### Using Command Line Arguments

```bash
npm run verify -- "0x283e3f25323d02dab..." "0x0d69b94acdfaca5ba..."
```

The first argument is the proof bytes and the second is the public input.

### Using Environment Variables

You can also set the proof and public input in your `.env` file:

```
PRIVATE_KEY=your_private_key_here
PROOF_BYTES=0x283e3f25323d02dab...
PUBLIC_INPUT=0x0d69b94acdfaca5ba...
```

And then run:

```bash
npm run verify
```

## Troubleshooting

### Gas Estimation Errors

If you encounter an "UNPREDICTABLE_GAS_LIMIT" error, it means ethers.js can't estimate the gas for the transaction because the contract is reverting. This script already sets a manual gas limit to try to bypass this issue.

If you still encounter problems, try the alternative Web3.js implementation:

```bash
npm run verify:web3
```

This alternative implementation:
1. Uses a different library (Web3.js instead of ethers.js)
2. Sets a manual gas limit
3. Provides more detailed error information

### Other Possible Issues:

1. **Contract Compatibility**: Ensure the proof and public input formats match what the contract expects
2. **Wallet Balance**: Ensure your wallet has enough Arbitrum Sepolia ETH for gas
3. **Correct Network**: Make sure you're connecting to the correct network (Arbitrum Sepolia)

## Contract Details

- **Contract Address**: 0xa7b9a263c5b8dbeb7143fab852504dbc58070489
- **Network**: Arbitrum Sepolia
- **RPC URL**: https://sepolia-rollup.arbitrum.io/rpc

## Functions

The contract has the following main functions:

- `verify(bytes proof_bytes, bytes public_input)` - Verifies a proof against the provided public input
- `last_verification_result()` - Returns the result of the last verification operation

## Example Proof and Public Input

The script includes an example proof and public input that should work with the deployed contract for testing purposes.