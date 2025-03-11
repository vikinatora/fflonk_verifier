const { ethers } = require('ethers');
require('dotenv').config();

// Configure the contract details
const contractAddress = '0xa7b9a263c5b8dbeb7143fab852504dbc58070489';
const rpcUrl = 'https://sepolia-rollup.arbitrum.io/rpc';

// ABI - only the functions we need
const abi = [
  {
    "inputs": [
      {
        "internalType": "bytes",
        "name": "proof_bytes",
        "type": "bytes"
      },
      {
        "internalType": "bytes",
        "name": "public_input",
        "type": "bytes"
      }
    ],
    "name": "verify",
    "outputs": [
      {
        "internalType": "bool",
        "name": "",
        "type": "bool"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "last_verification_result",
    "outputs": [
      {
        "internalType": "bool",
        "name": "",
        "type": "bool"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  }
];

// Sample values (these are from the contract example - you should replace with your actual values)
const exampleProofBytes = "0x283e3f25323d02dabdb94a897dc2697a3b930d8781381ec574af89a201a91d5a2c2808c59f5c736ff728eedfea58effc2443722e78b2eb4e6759a278e9246d600f9c56dc88e043ce0b90c402e96b1f4b1a246f4d0d69a4c340bc910e1f2fd80519e465e01bd7629f175931feed102cb6459a1be7b08018b93c142e961d0352d80b8e5d340df28c2f454c5a2535ca01a230bb945ee24b1171481a9a2c6496fed61cf8878e40adb52dc27da5e79718f118467319d15d64fed460d69d951376ac631a6c44faaec76e296b43fe720d700a63fd530f9064878b5f72f2ffe7458c2f031ac6ed8c1e0758dfb3702ed29bbc0c14b5e727c164b3ade07b9f164af0be54b0143b1a6534b2dcf2bd660e1b5b420d86c0c350fd9d614b639c5df98009f1375e141259679021d0a6a3aa3aae2516bace4a4a651265217ec0ea7c0d7f89b987100abcc93d98ff40bae16eff6c29955f7a37155bb25672b12eb5074dcb7c3e2b001718a257cca21ee593d1ba9f8e91e5168aed8e0b1893e11a6b583d975e747f8008a8c2150a04d8f867945ca1740dc3fc3b2fc4daff61b4725fb294435a1b90101803690ae70fc212b7e929de9a22a4642ef4772546cf93ffd1b1196a3d9113a3009c506755578932ca3630508ca1ed6ee83df5ec9e26cb0b5800a70967a1a93a04d142b6a532935a31d84f75d16929df6d38c3a210ac4f435a8024dfb7e6c1f3246d58038a943f237325b44f03d106e523adfec4324615a2dd09e1e5b9143b411c1cf09ee411cf9864d30df4904099920cee9ae8134d45dfeb29e46115d2e740098674b8fc2ca31fac6fcc9302860654fdc1b522b7e064b0759bc5924f332fa921121b5af880f83fbce02f19dabb8f684593e7322fb80bfc0d054797b1d4eff411b01bf68f81f2032ae4f7fc514bd76ca1b264f3989a92e6b3d74cda4f8a714920e4c02f5a71082a8bcf5be0b5750a244bd040a776ec541dfc2c8ae73180e9240ada5414d66387211eec80d7d9d48498efa1e646d64bb1bf8775b3796a9fd0bf0fdf8244018ce57b018c093e2f75ed77d8dbdb1a7b60a2da671de2efe5f6b9d7";
const examplePublicInput = "0x0d69b94acdfaca5bacc248a60b35b925a2374644ce0c1205db68228c8921d9d9";

async function verifyProof(proofBytes, publicInput) {
  try {
    // Connect to the Arbitrum Sepolia network
    const provider = new ethers.providers.JsonRpcProvider(rpcUrl);

    // You need a signer with funds to execute the transaction
    // Replace with your private key or use another way to get a signer
    const privateKey = process.env.PRIVATE_KEY;
    if (!privateKey) {
      console.error("Please set the PRIVATE_KEY environment variable");
      return;
    }
    
    const wallet = new ethers.Wallet(privateKey, provider);
    
    // Create a contract instance
    const contract = new ethers.Contract(contractAddress, abi, wallet);

    // Call the verify function with overrides to use manual gas limit
    console.log("Submitting verification transaction...");
    
    // Use a manual gas limit instead of relying on estimation
    // Arbitrum gas limit needs to be high enough for complex operations
    const overrides = {
      gasLimit: ethers.utils.hexlify(1000000), // Manually set a high gas limit
    };
    
    const tx = await contract.verify(proofBytes, publicInput, overrides);
    
    // Wait for the transaction to be mined
    console.log(`Transaction hash: ${tx.hash}`);
    console.log("Waiting for transaction confirmation...");
    const receipt = await tx.wait();
    
    console.log(`Transaction confirmed in block ${receipt.blockNumber}`);
    
    // Check the result
    const result = await contract.last_verification_result();
    console.log(`Verification result: ${result ? "Success" : "Failure"}`);
    
    return result;
  } catch (error) {
    console.error("Error verifying proof:", error);
    
    // More detailed error info if available
    if (error.error && error.error.message) {
      console.error("Contract error message:", error.error.message);
    }
    if (error.receipt) {
      console.error("Transaction receipt:", error.receipt);
    }
  }
}

// Add command-line parsing to make the script more flexible
async function main() {
  // Priority: 1. Command line args, 2. Environment variables, 3. Example values
  const proofBytes = process.argv[2] || process.env.PROOF_BYTES || exampleProofBytes;
  const publicInput = process.argv[3] || process.env.PUBLIC_INPUT || examplePublicInput;
  
  console.log("Using proof:", proofBytes.slice(0, 40) + "...");
  console.log("Using public input:", publicInput);
  
  await verifyProof(proofBytes, publicInput);
}

main().catch(console.error); 