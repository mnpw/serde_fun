// Custom serialize and serializer
// ```
// ┌────────────────┐     ┌─────────────┐      ┌─────────────┐
// │ Data structure │     │ Serde model │      │ Data format │
// ├────────────────┤     ├─────────────┤      ├─────────────┤
// │                │     │             │      │             │
// │     Enum       ◄─────►    bool     ◄──────►  BeamCode   │
// │                │     │             │      │             │
// └────────────────┘     └─────────────┘      └─────────────┘
//
// ```

// State::On (rust) -> true (serde) -> "s<1>" (beam_code)
// State::Off (rust) -> false (serde) -> "s<0>" (beam_code)

#![allow(unused)]
use serde::Serialize;

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
    // let on_state = State::On;
    // let json_state = serde_json::to_string(&on_state).unwrap();
    // let on_beam_state = beam_code::to_string(&on_state).unwrap();

    let on_beam_state = beam_code::to_string(&true).unwrap();

    println!("{on_beam_state}");

    // let off_state = State::Off;
    // let off_beam_state = beam_code::to_string(&off_state).unwrap();

    // println!("{off_beam_state}")
}

mod beam_code {
    use serde::{
        ser::{
            SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
            SerializeTupleStruct, SerializeTupleVariant,
        },
        Serialize, Serializer,
    };

    pub fn to_string<T>(value: &T) -> Result<String, ()>
    where
        T: ?Sized + Serialize,
    {
        let mut ser = BeamCode::new();
        let _ = value.serialize(&mut ser);

        Ok(ser.get_result())
    }

    struct BeamCode {
        result: String,
    }

    impl BeamCode {
        pub fn new() -> Self {
            BeamCode {
                result: String::new(),
            }
        }

        pub fn get_result(&self) -> String {
            self.result.to_string()
        }
    }

    impl<'a> Serializer for &'a mut BeamCode {
        type Ok = ();
        type Error = serde_json::Error;

        type SerializeSeq = SubSerializer;
        type SerializeTuple = SubSerializer;
        type SerializeTupleStruct = SubSerializer;
        type SerializeTupleVariant = SubSerializer;
        type SerializeMap = SubSerializer;
        type SerializeStruct = SubSerializer;
        type SerializeStructVariant = SubSerializer;

        fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
            self.result = match v {
                true => "s<1>".to_string(),
                false => "s<0>".to_string(),
            };

            Ok(())
        }

        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_newtype_struct<T: ?Sized>(
            self,
            name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn serialize_newtype_variant<T: ?Sized>(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            todo!()
        }

        fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            todo!()
        }

        fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            todo!()
        }

        fn serialize_tuple_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            todo!()
        }

        fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            todo!()
        }

        fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            todo!()
        }

        fn serialize_struct_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            todo!()
        }
    }

    struct SubSerializer;

    impl SerializeSeq for SubSerializer {
        type Ok = ();
        type Error = serde_json::Error;

        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    impl SerializeTuple for SubSerializer {
        type Ok = ();
        type Error = serde_json::Error;

        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    impl SerializeTupleStruct for SubSerializer {
        type Ok = ();
        type Error = serde_json::Error;

        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    impl SerializeTupleVariant for SubSerializer {
        type Ok = ();
        type Error = serde_json::Error;

        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    impl SerializeMap for SubSerializer {
        type Ok = ();
        type Error = serde_json::Error;

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }
    }

    impl SerializeStruct for SubSerializer {
        type Ok = ();
        type Error = serde_json::Error;

        fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    impl SerializeStructVariant for SubSerializer {
        type Ok = ();
        type Error = serde_json::Error;

        fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }
}
