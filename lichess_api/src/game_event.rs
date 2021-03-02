use serde::{Deserialize, Serialize};

use crate::model::{Perf, Player, State, Variant};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum GameEvent {
    GameFull {
        id: String,
        variant: Variant,
        speed: String,
        perf: Perf,
        rated: bool,
        #[serde(rename = "createdAt")]
        created_at: i64,
        #[serde(rename = "initialFen")]
        initial_fen: String,
        white: Player,
        black: Player,
        state: State,
    },
    GameState {
        #[serde(flatten)]
        state: State
    },
    ChatLine {},
}