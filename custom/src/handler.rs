use std::collections::HashMap;
use z4_types::{
    simple_game_result, Address, Error, HandleResult, Handler, MethodValues, PeerId, Player,
    Result, RoomId, Tasks,
};

#[derive(Clone)]
pub struct GameOperation {
    //
}

#[derive(Default)]
pub struct GameHandler {
    accounts: HashMap<PeerId, Player>,
    operations: Vec<GameOperation>,
}

#[async_trait::async_trait]
impl Handler for GameHandler {
    type Param = MethodValues;

    /// Viewable for game
    /// If true, when send message to all will also send to viewers
    /// If set false, no viwers
    fn viewable() -> bool {
        false
    }

    /// Accept params when submit to chain
    async fn chain_accept(_players: &[Player]) -> Vec<u8> {
        vec![]
    }

    /// Create new room when submmited success
    async fn chain_create(
        players: &[Player],
        _params: Vec<u8>,
        _rid: RoomId,
        _seed: [u8; 32],
    ) -> Option<(Self, Tasks<Self>)> {
        let accounts = players.iter().map(|p| (p.peer, *p)).collect();

        Some((
            Self {
                accounts,
                operations: vec![],
            },
            Default::default(),
        ))
    }

    /// Create new room from PoZK
    async fn pozk_create(
        player: Player,
        _params: Vec<u8>,
        _rid: RoomId,
    ) -> Option<(Self, Tasks<Self>)> {
        let mut accounts = HashMap::new();
        accounts.insert(player.peer, player);

        Some((
            Self {
                accounts,
                operations: vec![],
            },
            Default::default(),
        ))
    }

    /// New player join from PoZK
    async fn pozk_join(
        &mut self,
        player: Player,
        _params: Vec<u8>,
    ) -> Result<HandleResult<Self::Param>> {
        let mut result = HandleResult::default();

        self.accounts.insert(player.peer, player);
        if self.accounts.len() == 4 {
            result.started(); // !IMPORTANT: PoZK need started by handler
        }

        Ok(result)
    }

    /// When player online
    async fn online(&mut self, _player: PeerId) -> Result<HandleResult<Self::Param>> {
        Ok(HandleResult::default())
    }

    /// When player offline
    async fn offline(&mut self, _player: PeerId) -> Result<HandleResult<Self::Param>> {
        Ok(HandleResult::default())
    }

    /// Handle players' message in a room
    async fn handle(
        &mut self,
        _peer: PeerId,
        param: Self::Param,
    ) -> Result<HandleResult<Self::Param>> {
        let MethodValues { method, params } = param;

        // only support shoot method
        match method.as_str() {
            "echo" => {
                // build result
                let mut result = HandleResult::default();

                // broadcast method & params
                result.add_all(MethodValues::new("echo", params));

                // record operations
                self.operations.push(GameOperation {});

                // when game is over
                if self.operations.len() > 10 {
                    result.over();
                }

                Ok(result)
            }
            _ => Err(Error::Params),
        }
    }

    /// Generate proof for this game result, when find game is over
    async fn prove(&mut self) -> Result<(Vec<u8>, Vec<u8>)> {
        let players: Vec<Address> = self.accounts.values().map(|p| p.account).collect();
        let rank = simple_game_result(&players);
        let proof_bytes: Vec<u8> = vec![];

        Ok((rank, proof_bytes))
    }
}
