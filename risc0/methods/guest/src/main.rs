use risc0_zkvm::guest::env;
use ethers_core::{types::Address, abi::{encode, Token}};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub enum Operation {
    //
}

fn simple_game_result(ranks: &[Address]) -> Vec<u8> {
    encode(&[Token::Array(
        ranks.iter().map(|v| Token::Address(*v)).collect(),
    )])
}

fn main() {
    // read the input
    let operations: Vec<Operation> = env::read();

    let mut scores: HashMap<Address, u32> = HashMap::new();
    for op in operations {
        // TODO handle game operation
    }

    // write public output to the journal
    let mut players: Vec<(Address, u32)> = scores
        .iter()
        .filter_map(|(account, score)| if *score > 0 { Some((*account, *score)) } else { None })
        .collect();
    players.sort_by(|(_, sa), (_, sb)| sb.cmp(sa));
    let winners: Vec<Address> = players.iter().map(|(a, _s)| *a).collect();
    let rank = simple_game_result(&winners);

    env::commit(&rank);
    env::commit(env::cycle_count());
}
