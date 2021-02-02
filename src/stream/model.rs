use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub id: String,
    pub url: String,
    pub status: String,
    pub challenger: Player,
    pub dest_user: Player,
    pub variant: Variant,
    pub rated: bool,
    pub speed: String,
    pub time_control: TimeControl,
    pub color: String,
    pub perf: Perf,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub rating: i32,
    pub online: Option<bool>,
    pub provisional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub key: String,
    pub name: String,
    pub short: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum TimeControl {
    Unlimited {},
    Clock {
        limit: i32,
        increment: i32,
        show: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Perf {
    pub icon: Option<String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct State {
    moves: String,
    wtime: i32,
    btime: i32,
    winc: i32,
    binc: i32,
    wdraw: bool,
    bdraw: bool,
    status: String,
}