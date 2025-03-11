use fflonk_verifier::{verify, VerificationKey, Proof, Public, ProofData};
use hex_literal::hex;
use substrate_bn::arith::U256;
use std::convert::TryFrom;

/// This test demonstrates an alternative way to create and verify a proof
/// using the ProofData intermediate representation
#[test]
fn test_detailed_example() {
    // Create a proof data array with the same data as the README example
    // but demonstrating an alternative method using ProofData type
    let data_array = [
        // Convert the hex string from the README example to U256
        U256::from_slice(&hex!("283e3f25323d02dabdb94a897dc2697a3b930d8781381ec574af89a201a91d5a")).unwrap(),
        U256::from_slice(&hex!("2c2808c59f5c736ff728eedfea58effc2443722e78b2eb4e6759a278e9246d60")).unwrap(),
        U256::from_slice(&hex!("0f9c56dc88e043ce0b90c402e96b1f4b1a246f4d0d69a4c340bc910e1f2fd805")).unwrap(),
        U256::from_slice(&hex!("19e465e01bd7629f175931feed102cb6459a1be7b08018b93c142e961d0352d8")).unwrap(),
        U256::from_slice(&hex!("0b8e5d340df28c2f454c5a2535ca01a230bb945ee24b1171481a9a2c6496fed6")).unwrap(),
        U256::from_slice(&hex!("1cf8878e40adb52dc27da5e79718f118467319d15d64fed460d69d951376ac63")).unwrap(),
        U256::from_slice(&hex!("1a6c44faaec76e296b43fe720d700a63fd530f9064878b5f72f2ffe7458c2f03")).unwrap(),
        U256::from_slice(&hex!("1ac6ed8c1e0758dfb3702ed29bbc0c14b5e727c164b3ade07b9f164af0be54b0")).unwrap(),
        U256::from_slice(&hex!("143b1a6534b2dcf2bd660e1b5b420d86c0c350fd9d614b639c5df98009f1375e")).unwrap(),
        U256::from_slice(&hex!("141259679021d0a6a3aa3aae2516bace4a4a651265217ec0ea7c0d7f89b98710")).unwrap(),
        U256::from_slice(&hex!("0abcc93d98ff40bae16eff6c29955f7a37155bb25672b12eb5074dcb7c3e2b00")).unwrap(),
        U256::from_slice(&hex!("1718a257cca21ee593d1ba9f8e91e5168aed8e0b1893e11a6b583d975e747f80")).unwrap(),
        U256::from_slice(&hex!("08a8c2150a04d8f867945ca1740dc3fc3b2fc4daff61b4725fb294435a1b9010")).unwrap(),
        U256::from_slice(&hex!("1803690ae70fc212b7e929de9a22a4642ef4772546cf93ffd1b1196a3d9113a3")).unwrap(),
        U256::from_slice(&hex!("009c506755578932ca3630508ca1ed6ee83df5ec9e26cb0b5800a70967a1a93a")).unwrap(),
        U256::from_slice(&hex!("04d142b6a532935a31d84f75d16929df6d38c3a210ac4f435a8024dfb7e6c1f3")).unwrap(),
        U256::from_slice(&hex!("246d58038a943f237325b44f03d106e523adfec4324615a2dd09e1e5b9143b41")).unwrap(),
        U256::from_slice(&hex!("1c1cf09ee411cf9864d30df4904099920cee9ae8134d45dfeb29e46115d2e740")).unwrap(),
        U256::from_slice(&hex!("098674b8fc2ca31fac6fcc9302860654fdc1b522b7e064b0759bc5924f332fa9")).unwrap(),
        U256::from_slice(&hex!("21121b5af880f83fbce02f19dabb8f684593e7322fb80bfc0d054797b1d4eff4")).unwrap(),
        U256::from_slice(&hex!("11b01bf68f81f2032ae4f7fc514bd76ca1b264f3989a92e6b3d74cda4f8a7149")).unwrap(),
        U256::from_slice(&hex!("20e4c02f5a71082a8bcf5be0b5750a244bd040a776ec541dfc2c8ae73180e924")).unwrap(),
        U256::from_slice(&hex!("0ada5414d66387211eec80d7d9d48498efa1e646d64bb1bf8775b3796a9fd0bf")).unwrap(),
        U256::from_slice(&hex!("0fdf8244018ce57b018c093e2f75ed77d8dbdb1a7b60a2da671de2efe5f6b9d7")).unwrap(),
    ];

    // Convert the array to ProofData
    let proof_data = ProofData::from(data_array);
    
    // Convert ProofData to Proof
    let proof = Proof::try_from(proof_data).unwrap();
    
    // Create the verification key
    let vk = VerificationKey::default();
    
    // Create the public input (same as README example)
    let pubs_input = hex!("0d69b94acdfaca5bacc248a60b35b925a2374644ce0c1205db68228c8921d9d9");
    let pubs = Public::try_from(pubs_input.as_ref()).unwrap();

    // Verify the proof
    let result = verify(&vk, &proof, &pubs);
    
    // Assert that verification succeeds
    assert!(result.is_ok(), "Proof verification failed: {:?}", result);
    
    // Alternative method to create the public input directly from bytes
    let pubs_alt = Public::from(pubs_input);
    
    // Verify with the alternative public input creation
    let result_alt = verify(&vk, &proof, &pubs_alt);
    
    // Assert that verification succeeds
    assert!(result_alt.is_ok(), "Proof verification with alternative pubs failed: {:?}", result_alt);
} 