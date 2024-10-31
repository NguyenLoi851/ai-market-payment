use near_sdk::borsh::BorshSerialize;
use near_sdk::store::{ IterableMap, LookupMap };
use near_sdk::{
    env,
    ext_contract,
    near,
    near_bindgen,
    require,
    AccountId,
    BorshStorageKey,
    NearToken,
    PanicOnDefault,
};

pub const ZERO_TOKEN: NearToken = NearToken::from_yoctonear(0);

#[derive(BorshStorageKey, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub enum StorageKey {
    Models,
    FTDeposits,
}

#[allow(dead_code)]
#[ext_contract(ext_ft_contract)]
trait ExtFtContract {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: NearToken, memo: Option<String>);
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct ModelInfo {
    creator: AccountId,
    fee_per_prompt: NearToken,
    usage_count: u32,
    metadata_id: u32,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Payment {
    pub ft_id: AccountId,
    pub models: IterableMap<u32, ModelInfo>,
    pub ft_deposits: LookupMap<AccountId, NearToken>,
}

#[near_bindgen]
impl Payment {
    #[init]
    pub fn new(ft_id: AccountId) -> Self {
        Self {
            ft_id,
            models: IterableMap::new(StorageKey::Models),
            ft_deposits: LookupMap::new(StorageKey::FTDeposits),
        }
    }

    /// Register or update creator payment information
    pub fn register_model(&mut self, fee_per_prompt: NearToken, metadata_id: u32) {
        let creator = env::predecessor_account_id();

        // Check if the provided metadata_id already exists in any model
        for (_model_id, model_info) in self.models.iter() {
            if model_info.metadata_id == metadata_id {
                env::panic_str("Metadata ID already exists in another model.");
            }
        }

        let model_id = self.models.len() + 1;

        let fee_info = ModelInfo {
            creator,
            fee_per_prompt,
            usage_count: 0,
            metadata_id,
        };
        self.models.insert(model_id, fee_info);
        env::log_str("Model registered successfully.");
    }

    /// Get the number of model
    pub fn get_model_count(&self) -> u32 {
        self.models.len()
    }

    /// Get model information by its ID
    pub fn get_model_info(&self, model_id: u32) -> Option<ModelInfo> {
        self.models.get(&model_id).cloned()
    }

    /// Get a model by its metadata ID
    pub fn get_model_by_metadata_id(&self, metadata_id: u32) -> Option<(u32, ModelInfo)> {
        self.models.iter().find_map(|(model_id, model_info)| {
            if model_info.metadata_id == metadata_id {
                Some((*model_id, model_info.clone()))
            } else {
                None
            }
        })
    }

    /// Get all models by a creator
    pub fn get_models_by_creator(&self, creator: AccountId) -> Vec<(u32, ModelInfo)> {
        self.models
            .iter()
            .filter_map(|(model_id, model_info)| {
                if model_info.creator == creator {
                    Some((*model_id, model_info.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all models
    pub fn get_all_models(&self) -> Vec<(u32, ModelInfo)> {
        self.models
            .iter()
            .map(|(model_id, model_info)| (*model_id, model_info.clone()))
            .collect()
    }

    /// Update model information by its ID
    pub fn update_model_info(
        &mut self,
        model_id: u32,
        new_fee_per_prompt: Option<NearToken>,
        new_creator: Option<AccountId>
    ) {
        // Check if the model exists
        if let Some(fee_info) = self.models.get_mut(&model_id) {
            let caller = env::predecessor_account_id();

            // Check if the caller is the creator of the model
            if fee_info.creator != caller {
                env::log_str("Only the creator can update the model information.");
                return;
            }
            // Update fee_per_prompt if provided
            if let Some(fee) = new_fee_per_prompt {
                fee_info.fee_per_prompt = fee;
            }
            // Update creator if provided
            if let Some(wallet) = new_creator {
                fee_info.creator = wallet;
            }
            env::log_str("Model updated successfully.");
        } else {
            env::log_str("Model not found.");
        }
    }

    #[allow(dead_code, unused_variables)]
    pub fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: NearToken,
        msg: String
    ) -> NearToken {
        // get the contract ID which is the predecessor
        let ft_contract_id = env::predecessor_account_id();
        // Ensure only the specified FT can be used
        require!(ft_contract_id == self.ft_id, "FT contract ID does not match");

        //get the signer which is the person who initiated the transaction
        let signer_id = env::signer_account_id();

        //make sure that the signer isn't the predecessor. This is so that we're sure
        //this was called via a cross-contract call
        assert_ne!(
            ft_contract_id,
            signer_id,
            "nft_on_approve should only be called via cross-contract call"
        );
        //make sure the owner ID is the signer.
        assert_eq!(sender_id, signer_id, "owner_id should be signer_id");

        // Add the amount to the user's current balance
        let mut cur_bal = self.ft_deposits.get(&signer_id).unwrap_or(&ZERO_TOKEN);
        let binding = cur_bal.saturating_add(amount);
        cur_bal = &binding;
        self.ft_deposits.insert(signer_id, *cur_bal);

        // We don't return any FTs to the sender because we're storing all of them in their balance
        ZERO_TOKEN
    }

    pub fn ft_withdraw(&mut self, amount: NearToken) {
        let signer_id = env::signer_account_id();
        let mut cur_bal = self.ft_deposits.get(&signer_id).unwrap_or(&ZERO_TOKEN);

        assert!(cur_bal.ge(&amount), "Not enough FTs in deposit to cover withdraw: {:?}", amount);
        let binding = cur_bal.saturating_sub(amount);
        cur_bal = &binding;
        self.ft_deposits.insert(signer_id.clone(), *cur_bal);

        ext_ft_contract
            ::ext(self.ft_id.clone())
            // Attach 1 yoctoNEAR with static GAS equal to the GAS for nft transfer. Also attach an unused GAS weight of 1 by default.
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .ft_transfer(
                signer_id, //seller to transfer the FTs to
                amount, //amount to transfer
                Some("Withdraw from AI marketplace".to_string()) //memo (to include some context)
            );
    }

    pub fn ft_deposits_of(&self, account_id: AccountId) -> NearToken {
        *self.ft_deposits.get(&account_id).unwrap_or(&ZERO_TOKEN)
    }

    /// Pay for using model
    pub fn pay(&mut self, model_id: u32) {
        let buyer_id = env::predecessor_account_id();

        // Get the model and its price
        let mut model = self.models.get(&model_id).expect("Model not found.").clone();
        let price = model.fee_per_prompt;

        // Check if the buyer has enough balance
        let cur_bal = self.ft_deposits.get(&buyer_id).expect("No balance found for buyer.");
        assert!(cur_bal.ge(&price), "Not enough FTs in balance to cover payment: {:?}", price);

        // Deduct the amount from the buyer's balance
        self.ft_deposits.insert(buyer_id.clone(), cur_bal.saturating_sub(price));

        // Increment the usage count
        model.usage_count += 1;

        // Update the model info in the map
        self.models.insert(model_id, model.clone());

        // Log the payment action
        env::log_str(
            &format!(
                "User {} paid for using model {}. Current usage count: {}",
                buyer_id,
                model_id,
                model.usage_count
            )
        );

        ext_ft_contract
            ::ext(self.ft_id.clone())
            // Attach 1 yoctoNEAR with static GAS equal to the GAS for nft transfer. Also attach an unused GAS weight of 1 by default.
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .ft_transfer(
                model.creator, //seller to transfer the FTs to
                price, //amount to transfer
                Some("Fee from AI marketplace".to_string()) //memo (to include some context)
            );
    }
}

// /// Pay the creator for using their AI model
// #[payable]
// pub fn pay_creator(&mut self, creator_id: AccountId) {
//     let user_id = env::predecessor_account_id();
//     let deposit = env::attached_deposit();

//     if let Some(creator) = self.creators.get(&creator_id) {
//         assert!(
//             deposit >= creator.fee_per_prompt,
//             "Attached deposit is less than the required fee per prompt."
//         );

//         Promise::new(creator_id.clone()).transfer(deposit);
//         env::log_str(
//             &format!(
//                 "User {} paid {} to creator {} for using model {}.",
//                 user_id,
//                 deposit,
//                 creator_id,
//                 creator.model_id
//             )
//         );
//     } else {
//         env::panic_str("Creator not found.");
//     }
// }
