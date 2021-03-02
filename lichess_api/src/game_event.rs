use serde::{Deserialize, Serialize};
use crate::model::{Variant, Perf, Player, State};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum GameEvent {
    GameFull {
        id: String,
        variant: Variant,
        clock: Option<String>,
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