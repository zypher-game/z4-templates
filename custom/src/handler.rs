use std::collections::HashMap;
use z4_engine::{
    simple_game_result, Address, DefaultParams, Error, HandleResult, Handler, PeerId, Result,
    RoomId, Tasks,
};

#[derive(Default)]
pub struct Player {
    //
}

#[derive(Clone)]
pub struct Operation {
    //
}

#[derive(Default)]
pub struct GameHandler {
    accounts: HashMap<PeerId, (Address, Player)>,
    operations: Vec<Operation>,
}

#[async_trait::async_trait]
impl Handler for GameHandler {
    type Param = DefaultParams;

    /// accept params when submit to chain
    async fn accept(_peers: &[(Address, PeerId, [u8; 32])]) -> Vec<u8> {
        vec![]
    }

    /// create new room when submmited success
    async fn create(
        peers: &[(Address, PeerId, [u8; 32])],
        _params: Vec<u8>,
        _rid: RoomId,
    ) -> (Self, Tasks<Self>) {
        let accounts = peers
            .iter()
            .map(|(account, peer, _pk)| (*peer, (*account, Player {})))
            .collect();

        (
            Self {
                accounts,
                operations: vec![],
            },
            Default::default(),
        )
    }

    /// when player online
    async fn online(&mut self, _player: PeerId) -> Result<HandleResult<Self::Param>> {
        Ok(HandleResult::default())
    }

    /// when player offline
    async fn offline(&mut self, _player: PeerId) -> Result<HandleResult<Self::Param>> {
        Ok(HandleResult::default())
    }

    /// handle message in a room
    async fn handle(
        &mut self,
        _player: PeerId,
        method: &str,
        _params: DefaultParams,
    ) -> Result<HandleResult<Self::Param>> {
        // only support shoot method
        match method {
            "xxx" => {
                // build result
                let mut result = HandleResult::default();

                // broadcast method & params
                result.add_all("xxx", DefaultParams(vec![]));

                // record operations
                self.operations.push(Operation {});

                // check game is over and then do zkp
                let players: Vec<Address> = self.accounts.values().map(|(p, _)| *p).collect();
                let rank = simple_game_result(&players);
                let proof_bytes: Vec<u8> = vec![]; // TODO zkp with operation & rank
                result.over(rank, proof_bytes);

                Ok(result)
            }
            _ => Err(Error::Params),
        }
    }
}
