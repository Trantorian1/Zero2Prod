type SerOk = Option<(String, String)>;

pub struct EnvSerializer {
    path: String,
    seperator: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

impl Default for EnvSerializer {
    fn default() -> Self {
        Self { path: Default::default(), seperator: " ".to_string() }
    }
}

impl EnvSerializer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_separator(self, separator: &str) -> Self {
        Self { seperator: separator.to_string(), ..self }
    }

    pub fn with_prefix(self, prefix: &str) -> Self {
        Self { path: prefix.to_string(), ..self }
    }
}

impl<'a> serde::ser::Serializer for &'a mut EnvSerializer {
    type Ok = SerOk;
    type Error = Error;

    type SerializeSeq = serde::ser::Impossible<Self::Ok, Error>;
    type SerializeTuple = serde::ser::Impossible<Self::Ok, Error>;
    type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Error>;
    type SerializeMap = serde::ser::Impossible<Self::Ok, Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        match v {
            true => Ok(Some((self.path.clone(), "1".to_string()))),
            false => Ok(Some((self.path.clone(), "2".to_string()))),
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
        Ok(Some((self.path.clone(), v.to_string())))
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
        Ok(Some((self.path.clone(), v.to_string())))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Some((self.path.clone(), v.to_string())))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Some((self.path.clone(), v.to_string())))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Some((self.path.clone(), v.to_string())))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
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
        Ok(None)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(None)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
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

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
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

impl<'a> serde::ser::SerializeStruct for &'a mut EnvSerializer {
    type Ok = SerOk;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let path = self.path.clone();
        let path_new = if !self.path.is_empty() { format!("{path}_{key}") } else { key.to_string() };
        self.path = path_new;

        if let Ok(Some((k, v))) = value.serialize(&mut **self) {
            unsafe { std::env::set_var(&k.to_uppercase(), &v) };
        }

        self.path = path;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use serde::Serialize;

    use super::*;
    use crate::logs::fixtures::*;

    #[rstest::fixture]
    fn serializer() -> EnvSerializer {
        EnvSerializer::default()
    }

    #[rstest::rstest]
    fn serialize_struct(_logs: (), mut serializer: EnvSerializer) {
        #[derive(serde::Serialize)]
        struct Foo {
            bazz: u8,
        }

        let foo = Foo { bazz: 42 };
        foo.serialize(&mut serializer).expect("Failed serialization");

        assert_eq!(std::env::var("BAZZ").unwrap(), "42");
    }

    #[rstest::rstest]
    fn serialize_struct_nested(_logs: (), mut serializer: EnvSerializer) {
        #[derive(serde::Serialize)]
        struct Foo {
            bazz: Bazz,
        }

        #[derive(serde::Serialize)]
        struct Bazz {
            val: u8,
        }

        let foo = Foo { bazz: Bazz { val: 42 } };
        foo.serialize(&mut serializer).expect("Failed serialization");

        assert_eq!(std::env::var("BAZZ_VAL").unwrap(), "42");
    }

    #[rstest::rstest]
    fn serialize_prefix(_logs: (), mut serializer: EnvSerializer) {
        #[derive(serde::Serialize)]
        struct Foo {
            bazz: u8,
        }

        let foo = Foo { bazz: 42 };
        serializer = serializer.with_prefix("foo");
        foo.serialize(&mut serializer).expect("Failed serialization");

        assert_eq!(std::env::var("FOO_BAZZ").unwrap(), "42");
    }
}
