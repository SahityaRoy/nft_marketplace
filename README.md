# NFT market place on soroban 

# This code snippet demonstrates the implementation of a simple Non-Fungible Token (NFT) contract using the Soroban SDK in Rust. Here's a breakdown of what it does:

1. **NFT Definition**: Defines a struct `MyNFT` representing an NFT with fields for owner and metadata.

2. **Contract Implementation**: Implements a struct `MyNFTContract` to manage NFTs. It includes functions to mint new NFTs, transfer ownership, and retrieve NFT details.

3. **Trait Implementation**: Implements the `NFTContract` trait for `MyNFTContract`, specifying methods required for NFT functionality such as minting, transferring, and retrieval.

4. **Main Function**: Demonstrates the usage of the contract by minting a new NFT, retrieving it, transferring ownership, and verifying the transfer.

5. **Error Handling**: Utilizes `SorobanError` for custom error handling, such as when attempting to mint an NFT with an existing token ID or transferring an NFT without ownership.

This contract is creating and managing NFTs in Rust using the Soroban SDK, suitable for integration into blockchain applications or smart contracts.
