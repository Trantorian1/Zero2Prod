use serde::Serialize;

type SerOk = Option<(String, String)>;

#[tracing::instrument(skip_all)]
pub fn serialize_settings(settings: &crate::configuration::Settings) {
    tracing::trace!("App settings: {settings:#?}");
    let mut serializer = EnvSerializer::new(".env").unwrap().with_prefix("Z2P").with_separator(" ");
    settings.serialize(&mut serializer).expect("Failed to export settings to env");
}

#[derive(Clone)]
struct EnvSerializer {
    path: String,
    separator: String,
    writer: std::rc::Rc<std::cell::RefCell<std::io::BufWriter<std::fs::File>>>,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

impl serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

impl EnvSerializer {
    fn set_var(&mut self, k: &str, v: &str) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(self.writer.borrow_mut(), "{k}: {v}")
    }
}

impl EnvSerializer {
    fn new(path: &str) -> std::io::Result<Self> {
        let f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        let w = std::io::BufWriter::new(f);
        let c = std::rc::Rc::new(std::cell::RefCell::new(w));
        Ok(Self { path: Default::default(), separator: " ".to_string(), writer: c })
    }

    fn with_separator(self, separator: &str) -> Self {
        Self { separator: separator.to_string(), ..self }
    }

    fn with_prefix(self, prefix: &str) -> Self {
        Self { path: prefix.to_string(), ..self }
    }

    fn ser(&self, val: String) -> Result<SerOk, Error> {
        Ok(Some((self.path.clone(), val)))
    }
}

impl serde::ser::Serializer for &'_ mut EnvSerializer {
    type Ok = SerOk;
    type Error = Error;

    type SerializeSeq = EnvSerializerSeq;
    type SerializeTuple = EnvSerializerSeq;
    type SerializeTupleStruct = EnvSerializerSeq;
    type SerializeTupleVariant = EnvSerializerTupleVariant;
    type SerializeMap = EnvSerializerMap;
    type SerializeStruct = Self;
    type SerializeStructVariant = EnvSerializerStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        match v {
            true => self.ser("0".to_string()),
            false => self.ser("1".to_string()),
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.ser(v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.ser(v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.ser(v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.ser(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ser(v.to_string())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ser(hex::encode(v))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(None)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(None)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.ser(name.to_string())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.ser(variant.to_string())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self).map(|v| {
            v.map(|(k, v)| {
                let v = format!("{variant}_{v}");
                (k, v)
            })
        })
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(len
            .map(|n| EnvSerializerSeq { elems: Vec::with_capacity(n), serializer: self.clone() })
            .unwrap_or_else(|| EnvSerializerSeq { elems: Vec::default(), serializer: self.clone() }))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(EnvSerializerSeq { elems: Vec::with_capacity(len), serializer: self.clone() })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(EnvSerializerSeq { elems: Vec::with_capacity(len), serializer: self.clone() })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(EnvSerializerTupleVariant {
            variant: variant.to_string(),
            elems: Vec::with_capacity(len),
            serializer: self.clone(),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(EnvSerializerMap {
            elems: len.map(std::collections::HashMap::with_capacity).unwrap_or_default(),
            serializer: self.clone(),
        })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(EnvSerializerStructVariant { variant: variant.to_string(), serializer: self.clone() })
    }
}

struct EnvSerializerSeq {
    elems: Vec<String>,
    serializer: EnvSerializer,
}

impl serde::ser::SerializeSeq for EnvSerializerSeq {
    type Ok = SerOk;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(&mut &mut *self, value)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(&mut self)
    }
}

impl serde::ser::SerializeSeq for &'_ mut EnvSerializerSeq {
    type Ok = SerOk;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut self.serializer).map(|v| {
            if let Some((_, v)) = v {
                self.elems.push(v);
            }
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if !self.elems.is_empty() {
            let k = self.serializer.path.clone();
            let v = self.elems.join(&self.serializer.separator);
            self.serializer.set_var(&k, &v)?;
        }
        Ok(None)
    }
}

impl serde::ser::SerializeTuple for EnvSerializerSeq {
    type Ok = SerOk;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleStruct for EnvSerializerSeq {
    type Ok = SerOk;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

struct EnvSerializerTupleVariant {
    variant: String,
    elems: Vec<String>,
    serializer: EnvSerializer,
}

impl serde::ser::SerializeTupleVariant for EnvSerializerTupleVariant {
    type Ok = SerOk;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut self.serializer).map(|v| {
            if let Some((_, v)) = v {
                self.elems.push(v);
            }
        })
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        if !self.elems.is_empty() {
            let k = self.serializer.path.clone();
            let v = format!("{}_{}", self.variant, self.elems.join("_"));
            self.serializer.set_var(&k, &v)?;
        }
        Ok(None)
    }
}

struct EnvSerializerMap {
    elems: std::collections::HashMap<String, String>,
    serializer: EnvSerializer,
}

impl serde::ser::SerializeMap for EnvSerializerMap {
    type Ok = SerOk;
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        unimplemented!("Use serde::ser::SerializeMap::serialize_entry instead")
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        unimplemented!("Use serde::ser::SerializeMap::serialize_entry instead")
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + serde::Serialize,
        V: ?Sized + serde::Serialize,
    {
        let k = key.serialize(&mut self.serializer)?;
        let v = value.serialize(&mut self.serializer)?;

        if let (Some((_, k)), Some((_, v))) = (k, v) {
            self.elems.insert(k, v);
        }

        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        for (k, v) in self.elems.iter() {
            self.serializer.set_var(&format!("{}_{}", self.serializer.path, k), v)?;
        }

        Ok(None)
    }
}

impl serde::ser::SerializeStruct for &'_ mut EnvSerializer {
    type Ok = SerOk;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let path = self.path.clone();
        let path_new = if !self.path.is_empty() { format!("{path}_{key}") } else { key.to_string() };
        self.path = path_new;

        if let Some((k, v)) = value.serialize(&mut **self)? {
            self.set_var(&k, &v)?;
        }

        self.path = path;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(None)
    }
}

struct EnvSerializerStructVariant {
    variant: String,
    serializer: EnvSerializer,
}

impl serde::ser::SerializeStructVariant for EnvSerializerStructVariant {
    type Ok = SerOk;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if let Some((_, v)) = value.serialize(&mut self.serializer)? {
            self.serializer.set_var(&format!("{}_{}_{}", self.serializer.path, self.variant, key), &v)?;
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    // use serde::Serialize;
    //
    // use super::*;
    // use crate::logs::fixtures::*;
    //
    // #[rstest::fixture]
    // fn serializer() -> EnvSerializer {
    //     EnvSerializer::default()
    // }
    //
    // #[rstest::rstest]
    // fn serialize_unit_struct(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Bazz;
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Bazz,
    //     }
    //
    //     let foo = Foo { bazz: Bazz };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "Bazz");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_unit_variant(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     enum Bazz {
    //         A,
    //     }
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Bazz,
    //     }
    //
    //     let foo = Foo { bazz: Bazz::A };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "A");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_newtype_struct(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Bazz(u8);
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Bazz,
    //     }
    //
    //     let foo = Foo { bazz: Bazz(42) };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "42");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_newtype_variant(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     enum Bazz {
    //         A(u8),
    //     }
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Bazz,
    //     }
    //
    //     let foo = Foo { bazz: Bazz::A(42) };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "A_42");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_sequence(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Vec<u8>,
    //     }
    //
    //     let foo = Foo { bazz: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9] };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "0 1 2 3 4 5 6 7 8 9");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_sequence_sep(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Vec<u8>,
    //     }
    //
    //     let foo = Foo { bazz: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9] };
    //     serializer = serializer.with_separator(",");
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "0,1,2,3,4,5,6,7,8,9");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_sequence_empty(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Vec<u8>,
    //     }
    //
    //     let foo = Foo { bazz: vec![] };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap_err(), std::env::VarError::NotPresent);
    // }
    //
    // #[rstest::rstest]
    // fn serialize_tuple(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: (String, String),
    //     }
    //
    //     let foo = Foo { bazz: ("Hello".to_string(), "World".to_string()) };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "Hello World");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_tuple_sep(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: (String, String),
    //     }
    //
    //     let foo = Foo { bazz: ("Hello".to_string(), "World".to_string()) };
    //     serializer = serializer.with_separator(",");
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "Hello,World");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_tuple_unit(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: (),
    //     }
    //
    //     let foo = Foo { bazz: () };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap_err(), std::env::VarError::NotPresent);
    // }
    //
    // #[rstest::rstest]
    // fn serialize_tuple_struct(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Bazz(char, char, char);
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Bazz,
    //     }
    //
    //     let foo = Foo { bazz: Bazz('a', 'b', 'c') };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "a b c");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_tuple_struct_sep(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Bazz(char, char, char);
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Bazz,
    //     }
    //
    //     let foo = Foo { bazz: Bazz('a', 'b', 'c') };
    //     serializer = serializer.with_separator(",");
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "a,b,c");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_tuple_variant(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     enum Bazz {
    //         A(char, char, char),
    //     }
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Bazz,
    //     }
    //
    //     let foo = Foo { bazz: Bazz::A('a', 'b', 'c') };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "A_a_b_c");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_map(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: std::collections::HashMap<String, String>,
    //     }
    //
    //     let mut map = std::collections::HashMap::new();
    //     map.insert("Hello".to_string(), "World".to_string());
    //     map.insert("From".to_string(), "Trantorian".to_string());
    //
    //     let foo = Foo { bazz: map };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ_HELLO").unwrap(), "World");
    //     assert_eq!(std::env::var("BAZZ_FROM").unwrap(), "Trantorian");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_struct(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: u8,
    //     }
    //
    //     let foo = Foo { bazz: 42 };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ").unwrap(), "42");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_struct_nested(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Bazz,
    //     }
    //
    //     #[derive(serde::Serialize)]
    //     struct Bazz {
    //         val: u8,
    //     }
    //
    //     let foo = Foo { bazz: Bazz { val: 42 } };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ_VAL").unwrap(), "42");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_struct_variant(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     enum Bazz {
    //         ABC { a: char, b: char, c: char },
    //     }
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: Bazz,
    //     }
    //
    //     let foo = Foo { bazz: Bazz::ABC { a: 'a', b: 'b', c: 'c' } };
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("BAZZ_ABC_A").unwrap(), "a");
    //     assert_eq!(std::env::var("BAZZ_ABC_B").unwrap(), "b");
    //     assert_eq!(std::env::var("BAZZ_ABC_C").unwrap(), "c");
    // }
    //
    // #[rstest::rstest]
    // fn serialize_prefix(_logs: (), mut serializer: EnvSerializer) {
    //     #[derive(serde::Serialize)]
    //     struct Foo {
    //         bazz: u8,
    //     }
    //
    //     let foo = Foo { bazz: 42 };
    //     serializer = serializer.with_prefix("foo");
    //     let res = foo.serialize(&mut serializer).expect("Failed serialization");
    //
    //     assert_eq!(res, None);
    //     assert_eq!(std::env::var("FOO_BAZZ").unwrap(), "42");
    // }
}
