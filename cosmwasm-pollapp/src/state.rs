// JsonSchema allows structs to be serialized and deserialized to and from JSON
use schemars::JsonSchema;
// Deserialize and Serialize provide the serialization described above.
use serde::{Deserialize, Serialize};
// Addr is a Cosmos address, under the hood it is simply a string.
use cosmwasm_std::Addr;
// Item is a helper provided by storage plus. It effectively means we can store an item in storage.
// In this case, the STATE variable is an Item that stores a singular State struct.
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
    pub option: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

// A map with a String key and Poll value.
// The key will be a UUID generated clientside
pub const POLLS: Map<String, Poll> = Map::new("polls");

pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");
