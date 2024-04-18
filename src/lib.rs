use soroban_sdk::{NFTContract, SorobanError, TokenId};
use std::collections::HashMap;

// Define a struct to represent your NFT
#[derive(Debug, Clone)]
struct MyNFT {
    owner: String,
    metadata: String,
}

struct MyNFTContract {
    nfts: HashMap<TokenId, MyNFT>,
}

impl MyNFTContract {
    fn new() -> Self {
        Self {
            nfts: HashMap::new(),
        }
    }

    fn mint_nft(&mut self, owner: String, token_id: TokenId, metadata: String) -> Result<(), SorobanError> {
        if self.nfts.contains_key(&token_id) {
            return Err(SorobanError::Custom("Token ID already exists".to_string()));
        }

        let nft = MyNFT { owner, metadata };
        self.nfts.insert(token_id, nft);

        Ok(())
    }

    fn transfer_nft(&mut self, from: String, to: String, token_id: TokenId) -> Result<(), SorobanError> {
        if let Some(nft) = self.nfts.get_mut(&token_id) {
            if nft.owner != from {
                return Err(SorobanError::Custom("Not the owner of this NFT".to_string()));
            }

            nft.owner = to;
            Ok(())
        } else {
            Err(SorobanError::Custom("NFT not found".to_string()))
        }
    }

    fn get_nft(&self, token_id: TokenId) -> Option<&MyNFT> {
        self.nfts.get(&token_id)
    }
}

// Implement the NFTContract trait for your contract
impl NFTContract for MyNFTContract {
    type NFT = MyNFT;

    fn mint(&mut self, owner: String, token_id: TokenId, metadata: String) -> Result<(), SorobanError> {
        self.mint_nft(owner, token_id, metadata)
    }

    fn transfer(&mut self, from: String, to: String, token_id: TokenId) -> Result<(), SorobanError> {
        self.transfer_nft(from, to, token_id)
    }

    fn get_nft_by_id(&self, token_id: TokenId) -> Option<&Self::NFT> {
        self.get_nft(token_id)
    }
}

fn main() {
    // Create an instance of your contract
    let mut contract = MyNFTContract::new();
    // Mint a new NFT
    let owner = "Alice".to_string();
    let token_id = 1;
    let metadata = "Some metadata".to_string();
    contract.mint(owner.clone(), token_id, metadata.clone()).unwrap();

    // Get the NFT
    let nft = contract.get_nft(token_id).unwrap();
    println!("NFT: {:?}", nft);

    // Transfer the NFT
    let new_owner = "Bob".to_string();
    contract.transfer(owner, new_owner.clone(), token_id).unwrap();

    // Get the NFT after transfer
    let nft_after_transfer = contract.get_nft(token_id).unwrap();
    println!("NFT after transfer: {:?}", nft_after_transfer);
}
