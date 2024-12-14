#![no_std]

use sails_rs::{
    prelude::*,
    gstd::{
        calls::GStdRemoting,
        msg
    }
};

pub mod services;
pub mod states;
pub mod clients;

// use services::mini_dexs_service::MiniDexsService;
use services::vft_manager_service::VFTManagerService;
use clients::extended_vft_client::Vft as VftClient;

// use states::receiver_state::ReceiverState;

#[derive(Default)]
pub struct VFTManagerProgram;

#[program]
impl VFTManagerProgram {
    pub fn new() -> Self {
        VFTManagerService::<VftClient<GStdRemoting>>::seed(
            msg::source(), 
            None, 
            0,
            0,
            0
        );

        Self
    }

    pub fn new_with_data(
        vft_contract_id: Option<ActorId>,
        min_tokens_to_add: u128,
        max_tokens_to_burn: u128,
        tokens_per_vara: u128
    ) -> Self {
        VFTManagerService::<VftClient<GStdRemoting>>::seed(
            msg::source(), 
            vft_contract_id,
            min_tokens_to_add,
            max_tokens_to_burn,
            tokens_per_vara
        );

        Self
    }

    #[route("VFTManager")]
    pub fn vft_manager_svc(&self) -> VFTManagerService<VftClient<GStdRemoting>> {
        let vft_client = VftClient::new(GStdRemoting);
        VFTManagerService::new(vft_client)
    }
}