use serde::Serialize;

// Annotate a simple struct with Serialize and Deserialize macros
#[derive(Serialize)]
struct Point {
    x: i32,
    y: i32,
}

// Run cargo expand
fn main() {
    let p = Point { x: -1, y: -1 };

    let p_json = serde_json::to_string(&p).unwrap();
    // What we are doing here is:

    // let p = Point { x: -1, y: -1 };
    //
    // let writer = Vec::with_capacity(128);
    // let mut ser = Serializer::new(&mut writer)?;
    //
    // let mut serde_state = match serializer.serialize_struct("Point", 2)?;
    // serde_state.serialize_field("x", &self.x)?;
    // serde_state.serialize_field("y", &self.y)?;
    // serde_state.end();
    //
    // let p_json = String::from_utf8_unchecked(writer)?;

    println!("{p_json}");
}
