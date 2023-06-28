# Serializing and Deserializing
- A natural course of action in programming
- Serialization to store and move around data
- Deserialization to work with data in programs

```
 main.rs                              file.data
┌────────────────┐                   ┌────────────────┐
│                │    Serialize      │                │
│ struct A {     ├───────────────────► 01010100010101 │
│   x: usize     │                   │ 11101001101001 │
│ }              ◄───────────────────┤ 00011          │
│                │   Deserialize     │                │
│                │                   │                │
└────────────────┘                   └────────────────┘
```

# Serde
- a framework for serializing and deserializing Rust data structures 
    - efficiently
    - generically
- provides a layer (data model) between data structures and data formats

```
┌────────────────┐     ┌────────────┐      ┌─────────────┐
│ Data structure │     │ Data model │      │ Data format │
├────────────────┤     ├────────────┤      ├─────────────┤
│                │     │            │      │             │
│     Enum       │     │            │      │    JSON     │
│     Struct     ◄─────►   Serde    ◄──────►    TOML     │
│     usize      │     │            │      │    YAML     │
│                │     │            │      │             │
└────────────────┘     └────────────┘      └─────────────┘
```

# Serialization
```
                   impl                       impl
┌────────────────┐ Serialize  ┌────────────┐  Serializer  ┌─────────────┐
│ Data structure ├────────────► Data model ├──────────────► Data format │
└────────────────┘            └────────────┘              └─────────────┘
```

- serde defines `Serialize` and `Serializer` traits
- `Serialize` defines how a data structure is translated to the serde data model
- `Serializer` defines how the serde data model is serialized

## `Serialize`
```rust
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
```
- serde provides `impl Serialize` for primitive rust types
```rust
macro_rules! primitive_impl {
    ($ty:ident, $method:ident $($cast:tt)*) => {
        impl Serialize for $ty {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.$method(*self $($cast)*)
                }
        }
    }
}

primitive_impl!(bool, serialize_bool);
primitive_impl!(isize, serialize_i64 as i64);
primitive_impl!(i8, serialize_i8);
...
// impl Serialize for usize {
//     #[inline]
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_u64(*self as u64)
//         }
// }
primitive_impl!(usize, serialize_u64 as u64);
...
```
- serde_derive proc macro provides `Serialize` implementations for structs and enums in the program
```rust
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

// This is what #[derive(Serialize)] would generate.
impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Notice that `Serialize` defines that `Person` has to be taken as struct (`.serialize_struct`)
        // This is what it means to convert data structure (`Person`) to be represented in data model (`struct`)
        let mut s = serializer.serialize_struct("Person", 3)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("age", &self.age)?;
        s.serialize_field("phones", &self.phones)?;
        s.end()
    }
}
```

## `Serializer`
- serde only defines this trait but does not implement it
```rust
pub trait Serializer: Sized {
    type Ok;
    type Error: Error;
    ...
    type SerializeMap: SerializeMap<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStruct: SerializeStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStructVariant: SerializeStructVariant<Ok = Self::Ok, Error = Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;
    ...
    ...
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>;
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>;
}
```
- crates like `serde_json`, `serde_yaml` do the implementation
```rust
// See serde_json::ser module
// Note: This is simplified 

/// A structure for serializing Rust values into JSON.

pub struct Serializer {
    writer: W,
    formatter: F,
}

impl ser::Serializer for Serializer

{
    ...
    fn serialize_bool(self, value: bool) -> Result<()> {
        self.formatter.write_bool(&mut self.writer, value).map_err(Error::io)
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        self.formatter.write_i8(&mut self.writer, value).map_err(Error::io)
    }
    ...
}
```

```rust
/// This trait abstracts away serializing the JSON control characters, which allows the user to
/// optionally pretty print the JSON output.

pub trait Formatter {
    ...
    /// Writes a `null` value to the specified writer.
    fn write_null(&mut self, writer: W) -> io::Result<()> {
        writer.write_all(b"null")
    }

    /// Writes a `true` or `false` value to the specified writer.
    fn write_bool(&mut self, writer: &mut W, value: bool) -> io::Result<()> {
        let s = if value {
            b"true" as &[u8]
        } else {
            b"false" as &[u8]
        };
        writer.write_all(s)
    }
    ...
}
```

# Deserialization
```
                   impl                         impl
┌────────────────┐ Deserialize  ┌────────────┐  Deserializer  ┌─────────────┐
│ Data structure ◄──────────────┤ Data model ◄────────────────┤ Data format │
└────────────────┘              └────────────┘                └─────────────┘
```

- serde defines `Deserialize` and `Deserializer` traits

## `Deserialize`
```rust
pub trait Deserialize<'de>: Sized {
    /// Deserialize this value from the given Serde deserializer.
    ///
    /// See the [Implementing `Deserialize`][impl-deserialize] section of the
    /// manual for more information about how to implement this method.
    ///
    /// [impl-deserialize]: https://serde.rs/impl-deserialize.html
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
```
