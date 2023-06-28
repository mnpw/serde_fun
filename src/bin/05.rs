// Note: This part does not work

// Custom serialize
// ```
// ┌────────────────┐     ┌─────────────┐      ┌─────────────┐
// │ Data structure │     │ Serde model │      │ Data format │
// ├────────────────┤     ├─────────────┤      ├─────────────┤
// │                │     │             │      │             │
// │     Enum       ◄─────►    bool     ◄──────►    JSON     │
// │                │     │             │      │             │
// └────────────────┘     └─────────────┘      └─────────────┘
// ```

use std::fmt;

use serde::{
    de::{Error, Unexpected},
    Deserialize, Serialize,
};

#[derive(Debug)]
enum State {
    On,
    Off,
}

// impl Serialize for State {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match self {
//             State::On => true.serialize(serializer),
//             State::Off => false.serialize(serializer),
//         }
//     }
// }

struct Visitor;

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "bad input received")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v {
            "s<0>" => Ok(false),
            "s<1>" => Ok(true),
            _ => {
                return Err(serde::de::Error::invalid_type(
                    Unexpected::Other("bad input"),
                    &self,
                ))
            }
        }
    }
}

impl<'de> Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let t = deserializer.deserialize_bool(Visitor)?;

        match t {
            true => Ok(State::On),
            false => Ok(State::Off),
        }
    }
}

fn main() {
    let on_state = "s<0>";
    let json_state: bool = serde_json::from_str(&on_state).unwrap();

    println!("{json_state:?}")
}
