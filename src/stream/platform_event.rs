use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Event {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    id: String,
    url: String,
    status: String,
    challenger: Player,
    dest_user: Player,
    variant: Variant,
    rated: bool,
    speed: String,
    time_control: TimeControl,
    color: String,
    perf: Perf,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    id: String,
    name: String,
    title: Option<String>,
    rating: i32,
    provisional: bool,
    online: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    key: String,
    name: String,
    short: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeControl {
    #[serde(rename = "type")]
    control_type: String,
    limit: i32,
    increment: i32,
    show: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Perf {
    icon: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    id: String,
}