// Note: this is NOT the exact output, I've simplified it
use serde::{Deserialize, Serialize};
struct Point {
    x: i32,
    y: i32,
}

// Why the Impl in a const block?
// So we don't pollute the namespace with
// structs, types, functions that we temporarily create
// for defining impl
// see: https://internals.rust-lang.org/t/anonymous-modules/15441

const _: () = {
    extern crate serde as _serde;
    use _serde::{ser, Serializer};

    impl _serde::Serialize for Point {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // we get a `SerializeStruct` type here
            let mut serde_state = match serializer.serialize_struct("Point", false as usize + 1 + 1)
            {
                Ok(val) => val,
                Err(err) => {
                    return Err(err);
                }
            };

            match ser::SerializeStruct::serialize_field(&mut serde_state, "x", &self.x) {
                Ok(val) => val,
                Err(err) => {
                    return Err(err);
                }
            };
            match ser::SerializeStruct::serialize_field(&mut serde_state, "y", &self.y) {
                Ok(val) => val,
                Err(err) => {
                    return Err(err);
                }
            };
            ser::SerializeStruct::end(serde_state)
        }
    }
};

fn main() {
    let p = Point { x: -1, y: -1 };
}
