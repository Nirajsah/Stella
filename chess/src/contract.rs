#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use chess::Operation;
use linera_sdk::{
    base::WithContractAbi,
    views::{RootView, View, ViewStorageContext},
    Contract, ContractRuntime,
};

use self::state::Chess;

#[allow(dead_code)]
pub struct ChessContract {
    state: Chess,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(ChessContract);

impl WithContractAbi for ChessContract {
    type Abi = chess::ChessAbi;
}

impl Contract for ChessContract {
    type Message = ();
    type Parameters = ();
    type InstantiationArgument = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Chess::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        ChessContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {}

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::NewGame => {
                let board = chess_rs::ChessBoard::new().get_board();
                self.state
                    .value
                    .insert(&1, chess::Board { board: board })
                    .expect("Failed to insert board");
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
