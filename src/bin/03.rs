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

use serde::Serialize;

// #[derive(Serialize)]
enum State {
    On,
    Off,
}

impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            State::On => true.serialize(serializer),
            State::Off => false.serialize(serializer),
        }
    }
}

fn main() {
    let result = serde_json::to_string(&State::On).unwrap();

    println!("{result}")
}
