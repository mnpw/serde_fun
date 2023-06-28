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
}
