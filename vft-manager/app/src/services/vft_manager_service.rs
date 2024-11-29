use sails_rs::calls::{Call, Query};
use sails_rs::{
    prelude::*,
    gstd::msg,
};
use gstd::exec;

use crate::states::vft_manager_state::VFTManagerState;
use crate::clients::extended_vft_client::traits::Vft;

static mut VFT_MANAGER_STATE: Option<VFTManagerState> = None;

const ONE_TVARA: u128 = 1_000_000_000_000; // Value of one TVara and Vara

pub struct VFTManagerService<VftClient> {
    pub vft_client: VftClient
}

// #[service]
impl<VftClient> VFTManagerService<VftClient> 
where VftClient: Vft
{
    pub fn seed(
        admin: ActorId,
        vft_contract_id: Option<ActorId>,
        min_tokens_to_add: u128,
        tokens_per_vara: u128
    ) {
        unsafe {
            VFT_MANAGER_STATE = Some(
                VFTManagerState {
                    admins: vec![admin],
                    vft_contract_id,
                    min_tokens_to_add,
                    tokens_per_vara
                }
            );
        };
    }

    pub fn new(
        vft_client: VftClient
    ) -> Self {
        Self {
            vft_client
        }
    }

    // ## Add new a new admin
    // Only admins can add others admins
    pub fn add_admin(&mut self, new_admin_address: ActorId) -> VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        state.admins.push(new_admin_address);

        VFTManagerEvents::NewAdminAdded(new_admin_address)
    }

    // ## Change vft contract id
    // Only the contract owner can perform this action
    pub fn set_vft_contract_id(&mut self, vft_contract_id: ActorId) -> VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        state.vft_contract_id = Some(vft_contract_id);

        VFTManagerEvents::VFTContractIdSet
    }

    // ## Change the minimum number of tokens to add to the contract
    // Only the contract owner can perform this action
    pub fn set_min_tokens_to_add(&mut self, min_tokens_to_add: u128) -> VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        state.min_tokens_to_add = min_tokens_to_add;

        VFTManagerEvents::MinTokensToAddSet
    }

    // ## Change the number of tokens to exchange for one rod
    // Only the contract owner can perform this action
    pub fn set_tokens_per_vara(&mut self, tokens_per_vara: u128) -> VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        state.tokens_per_vara = tokens_per_vara;

        VFTManagerEvents::SetTokensPerVaras
    }


    // ## Add an amount of tokens to the vft contract for this contract
    // Only the contract owner can perform this action
    pub async fn add_tokens_to_contract(&mut self, tokens_to_add: u128) ->  VFTManagerEvents {
        let state = self.state_mut();
        let caller = msg::source();

        if state.is_admin(&caller) {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OnlyAdminsCanDoThatAction
            );
        }

        if state.vft_contract_id.is_none() {
            return VFTManagerEvents::Error(
                VFTManagerErrors::VftContractIdNotSet
            );
        }

        if tokens_to_add < state.min_tokens_to_add {
            return VFTManagerEvents::Error(
                VFTManagerErrors::MinTokensToAdd(state.min_tokens_to_add)
            );
        }

        let result = self
            .add_num_of_tokens_to_contract(
                tokens_to_add, 
                state.vft_contract_id.unwrap()
            )
            .await;

        if let Err(error_variant) = result {
            return VFTManagerEvents::Error(error_variant);
        }

        VFTManagerEvents::TokensAdded
    }

    // ## Swap Varas for tokens
    // Receive a certain amount of varas and then make a swap for a certain number of tokens
    pub async fn swap_tokens_by_num_of_varas(&mut self) -> VFTManagerEvents {
        let value = msg::value();
        let caller = msg::source();

        if value == 0 {
            return VFTManagerEvents::Error(
                VFTManagerErrors::CantSwapTokensWithAmount {
                    min_amount: 1,
                    actual_amount: 0
                }
            );
        }

        let state = self.state_ref();

        if state.vft_contract_id.is_none() {
            return VFTManagerEvents::Error(
                VFTManagerErrors::VftContractIdNotSet
            );
        }

        let num_of_tvaras = value / ONE_TVARA;
        let tokens: u128 = num_of_tvaras * state.tokens_per_vara;

        let total_tokens_supply = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(state.vft_contract_id.unwrap())
            .await;

        let Ok(total_suply) = total_tokens_supply else {
            return VFTManagerEvents::Error(
                VFTManagerErrors::ErrorInVFTContract
            );
        };

        if total_suply < U256::from(tokens) {
            msg::send(
                msg::source(),
                VFTManagerEvents::RefundOfVaras(num_of_tvaras), 
                value
            )
            .expect("Error sending message");

            return VFTManagerEvents::RefundOfVaras(num_of_tvaras);
        }

        let response = self
            .vft_client
            .transfer(caller, U256::from(tokens))
            .send_recv(state.vft_contract_id.unwrap())
            .await;

        let Ok(transfer_status) = response else {
            return VFTManagerEvents::Error(
                VFTManagerErrors::ErrorInVFTContract
            );
        };

        if !transfer_status {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OperationWasNotPerformed
            );
        }

        VFTManagerEvents::TokensSwapSuccessfully {
            total_tokens: tokens,
            total_varas: num_of_tvaras
        }
    }

    /// ## Swap tokens for Varas
    pub async fn swap_tokens_to_varas(&mut self, amount_of_tokens: u128) -> VFTManagerEvents {
        let state = self.state_ref();

        if amount_of_tokens < state.tokens_per_vara {
            return VFTManagerEvents::Error(
                VFTManagerErrors::CantSwapTokensWithAmount {
                    min_amount: state.tokens_per_vara,
                    actual_amount: amount_of_tokens
                }
            );
        } 

        let varas_to_send = amount_of_tokens / state.tokens_per_vara;
        let amount_of_tokens = varas_to_send * state.tokens_per_vara;
        let total_tokens_to_swap: U256 = U256::from(amount_of_tokens);
        let caller = msg::source();


        if state.vft_contract_id.is_none() {
            return VFTManagerEvents::Error(
                VFTManagerErrors::VftContractIdNotSet
            );
        }

        let response = self
            .vft_client
            .balance_of(caller)
            .recv(state.vft_contract_id.unwrap())
            .await;

        let Ok(user_total_tokens) = response else {
            return VFTManagerEvents::Error(
                VFTManagerErrors::ErrorInVFTContract
            );
        };

        if user_total_tokens < total_tokens_to_swap {
            return VFTManagerEvents::Error(
                VFTManagerErrors::CantSwapUserTokens { 
                    user_tokens: user_total_tokens, 
                    tokens_to_swap: total_tokens_to_swap 
                }
            );
        }

        let response = self
            .vft_client
            .burn(caller, total_tokens_to_swap)
            .send_recv(state.vft_contract_id.unwrap())
            .await;

        let Ok(operation_result) = response else {
            return VFTManagerEvents::Error(
                VFTManagerErrors::ErrorInVFTContract
            );
        };

        if !operation_result {
            return VFTManagerEvents::Error(
                VFTManagerErrors::OperationWasNotPerformed
            );
        }

        let result = self   
            .add_num_of_tokens_to_contract(
                amount_of_tokens, 
                state.vft_contract_id.unwrap()
            )
            .await;

        if let Err(error_variant) = result {
            return VFTManagerEvents::Error(error_variant);
        }

        msg::send(
            caller, 
            VFTManagerEvents::TotalSwapInVaras(varas_to_send), 
            varas_to_send * ONE_TVARA
        )
        .expect("Error sending message");

        VFTManagerEvents::TokensSwapSuccessfully { total_tokens: amount_of_tokens, total_varas: varas_to_send }
    }



    /// ## Varas stored in contract
    pub fn contract_total_varas_stored(&self) -> VFTManagerQueryEvents {
        VFTManagerQueryEvents::ContractBalanceInVaras(exec::value_available() / ONE_TVARA)
    }

    /// ## Returns the total number of tokens in the contract (In U256 format)
    pub async fn total_tokens_to_swap(&self) -> VFTManagerQueryEvents {
        let state = self.state_mut();

        if state.vft_contract_id.is_none() {
            return VFTManagerQueryEvents::Error(
                VFTManagerErrors::VftContractIdNotSet
            );
        }

        let response = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(state.vft_contract_id.unwrap())
            .await;

        let Ok(total_tokens_to_swap) = response else {
            return VFTManagerQueryEvents::Error(
                VFTManagerErrors::ErrorInVFTContract
            );
        };

        VFTManagerQueryEvents::TotalTokensToSwap(total_tokens_to_swap)
    }

    /// ## Returns the total number of tokens in the contract (In u128 format)
    pub async fn total_tokens_to_swap_as_u128(&self) -> VFTManagerQueryEvents {
        let state = self.state_mut();

        if state.vft_contract_id.is_none() {
            return VFTManagerQueryEvents::Error(
                VFTManagerErrors::VftContractIdNotSet
            );
        }

        let response = self
            .vft_client
            .balance_of(exec::program_id())
            .recv(state.vft_contract_id.unwrap())
            .await;

        let Ok(total_tokens_to_swap) = response else {
            return VFTManagerQueryEvents::Error(
                VFTManagerErrors::ErrorInVFTContract
            );
        };

        VFTManagerQueryEvents::TotalTokensToSwapAsU128(total_tokens_to_swap.as_u128())
    }

    pub fn tokens_to_swap_one_vara(&self) -> VFTManagerQueryEvents {
        VFTManagerQueryEvents::TokensToSwapOneVara(self.state_ref().tokens_per_vara)
    }

    /// ## Send an amount of tokens to the vft contract
    async fn add_num_of_tokens_to_contract(&mut self, tokens_to_add: u128, vft_contract_id: ActorId) -> Result<(), VFTManagerErrors> {
        let response = self
            .vft_client
            .mint(
                exec::program_id(), 
                U256::from(tokens_to_add)
            )
            .send_recv(vft_contract_id)
            .await;

        let Ok(operation_result) = response else {
            return Err(VFTManagerErrors::ErrorInVFTContract);
        };

        if !operation_result {
            return Err(VFTManagerErrors::OperationWasNotPerformed);
        }

        Ok(())
    }

    fn state_mut(&self) -> &'static mut VFTManagerState {
        let state = unsafe { VFT_MANAGER_STATE.as_mut() };
        debug_assert!(state.is_none(), "state is not started!");
        unsafe { state.unwrap_unchecked() }
    }

    fn state_ref(&self) -> &'static VFTManagerState {
        let state = unsafe { VFT_MANAGER_STATE.as_ref() };
        debug_assert!(state.is_none(), "state is not started!");
        unsafe { state.unwrap_unchecked() }
    }
}


#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum VFTManagerQueryEvents {
    ContractBalanceInVaras(u128),
    UserTotalTokensAsU128(u128),
    UserTotalTokens(U256),
    TotalTokensToSwap(U256),
    TotalTokensToSwapAsU128(u128),
    TokensToSwapOneVara(u128),
    NumOfTokensForOneVara(u128),
    Error(VFTManagerErrors)
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum VFTManagerEvents {
    NewAdminAdded(ActorId),
    RefundOfVaras(u128),
    VFTContractIdSet,
    MinTokensToAddSet,
    TokensAdded,
    SetTokensPerVaras,
    TotalSwapInVaras(u128),
    TokensSwapSuccessfully {
        total_tokens: u128,
        total_varas: u128
    },
    Error(VFTManagerErrors)
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum VFTManagerErrors {
    MinTokensToAdd(u128),
    CantSwapTokens {
        tokens_in_vft_contract: U256
    }, 
    CantSwapUserTokens {
        user_tokens: U256,
        tokens_to_swap: U256
    },
    ContractCantMint,
CantSwapTokensWithAmount {
        min_amount: u128,
        actual_amount: u128
    },
    OnlyAdminsCanDoThatAction,
    VftContractIdNotSet,
    ErrorInVFTContract,
    ErrorInGetNumOfVarasToSwap,
    OperationWasNotPerformed
}
