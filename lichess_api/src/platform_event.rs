use serde::{Deserialize, Serialize};

use crate::model::{Challenge, Game};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum PlatformEvent {
    GameStart {
        game: Game,
    },
    GameFinish {
        game: Game,
    },
    Challenge {
        challenge: Challenge,
    },
    ChallengeCanceled {
        challenge: Challenge,
    },
    ChallengeDeclined {
        challenge: Challenge,
    },
}