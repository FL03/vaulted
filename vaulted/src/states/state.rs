/*
    Appellation: state <states>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::States;
use scsys::prelude::{Message, Stateful, Timestamp};
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub message: Message,
    pub state: States,
    pub timestamp: i64,
}

impl State {
    pub fn new(message: Message, state: States) -> Self {
        let timestamp = Timestamp::default().into();
        Self {
            message,
            state,
            timestamp,
        }
    }
    pub fn state(&self) -> &States {
        &self.state
    }
    pub fn update_message(&mut self, data: serde_json::Value) -> &Self {
        self.message.push(data);
        self.update_timestamp()
    }
    pub fn update_state(&mut self, state: States) -> &Self {
        self.state = state;
        self.update_timestamp()
    }
    fn update_timestamp(&mut self) -> &Self {
        self.timestamp = Timestamp::default().into();
        self
    }
}

impl Stateful for State {
    type Data = serde_json::Value;

    fn message(&self) -> &Message<Self::Data> {
        &self.message
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(Message::default(), States::default())
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl From<Message> for State {
    fn from(data: Message) -> Self {
        Self::new(data, Default::default())
    }
}

impl From<States> for State {
    fn from(data: States) -> Self {
        Self::new(Default::default(), data)
    }
}

impl From<State> for Message {
    fn from(data: State) -> Self {
        data.message
    }
}

impl From<State> for States {
    fn from(data: State) -> Self {
        data.state
    }
}
