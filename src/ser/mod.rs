//! Serialize a Rust data structure into JSON data

use core::fmt;

use serde::ser;
use serde::ser::SerializeStruct as _;

mod map;
mod sequence;
mod struct_;
mod unreachable;

use self::map::SerializeMap;
use self::sequence::SerializeSeq;
use self::struct_::{SerializeStruct, SerializeStructVariant};
use self::unreachable::Unreachable;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Element {
    SequenceEntry,
    PreMappingKey,
    MappingKey,
    Literal,
    None,
}

pub(crate) struct Serializer<W: fmt::Write> {
    writer: W,
    depth: usize,
    preceding_element: Element,
}

impl<W: fmt::Write> Serializer<W> {
    fn char(&mut self, c: char) -> fmt::Result {
        self.writer.write_char(c)
    }

    fn str(&mut self, string: &str) -> fmt::Result {
        self.writer.write_str(string)
    }

    fn indent(&mut self, mut element: Element) -> fmt::Result {
        match (self.preceding_element, element) {
            (Element::SequenceEntry, _) | (Element::MappingKey, Element::Literal) => {
                self.char(' ')?
            }
            (Element::PreMappingKey, Element::Literal) => element = Element::MappingKey,
            (Element::Literal, Element::MappingKey) => (),
            (Element::None, _) => write!(self.writer, "{:indent$}", "", indent = self.depth * 2)?,
            _ => write!(self.writer, "\n{:indent$}", "", indent = self.depth * 2)?,
        }
        self.preceding_element = element;
        Ok(())
    }

    fn push(&mut self) {
        self.depth += 1;
    }

    fn pop(&mut self) {
        self.depth -= 1;
        self.preceding_element = Element::None;
    }
}

/// Upper-case hex for value in 0..16, encoded as ASCII bytes
impl<'a, W: fmt::Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = fmt::Error;
    type SerializeSeq = SerializeSeq<'a, W>;
    type SerializeTuple = SerializeSeq<'a, W>;
    type SerializeTupleStruct = Unreachable;
    type SerializeTupleVariant = Unreachable;
    type SerializeMap = SerializeMap<'a, W>;
    type SerializeStruct = SerializeStruct<'a, W>;
    type SerializeStructVariant = SerializeStructVariant<'a, W>;

    fn serialize_bool(self, v: bool) -> fmt::Result {
        self.indent(Element::Literal)?;
        self.str(if v { "true" } else { "false" })
    }

    fn serialize_i8(self, v: i8) -> fmt::Result {
        self.indent(Element::Literal)?;
        write!(self.writer, "{}", v)
    }

    fn serialize_i16(self, v: i16) -> fmt::Result {
        self.indent(Element::Literal)?;
        write!(self.writer, "{}", v)
    }

    fn serialize_i32(self, v: i32) -> fmt::Result {
        self.indent(Element::Literal)?;
        write!(self.writer, "{}", v)
    }

    fn serialize_i64(self, v: i64) -> fmt::Result {
        self.indent(Element::Literal)?;
        write!(self.writer, "{}", v)
    }

    fn serialize_u8(self, v: u8) -> fmt::Result {
        self.indent(Element::Literal)?;
        write!(self.writer, "{}", v)
    }

    fn serialize_u16(self, v: u16) -> fmt::Result {
        self.indent(Element::Literal)?;
        write!(self.writer, "{}", v)
    }

    fn serialize_u32(self, v: u32) -> fmt::Result {
        self.indent(Element::Literal)?;
        write!(self.writer, "{}", v)
    }

    fn serialize_u64(self, v: u64) -> fmt::Result {
        self.indent(Element::Literal)?;
        write!(self.writer, "{}", v)
    }

    fn serialize_f32(self, v: f32) -> fmt::Result {
        self.indent(Element::Literal)?;
        self.str(ryu::Buffer::new().format(v))
    }

    fn serialize_f64(self, v: f64) -> fmt::Result {
        self.indent(Element::Literal)?;
        self.str(ryu::Buffer::new().format(v))
    }

    fn serialize_char(self, c: char) -> fmt::Result {
        self.indent(Element::Literal)?;
        self.char(c)
    }

    fn serialize_str(self, v: &str) -> fmt::Result {
        self.indent(Element::Literal)?;
        let mut quote = false;
        match v.chars().next().unwrap_or(' ') {
            '{' | '[' | ' ' | '&' | '*' | '#' | ',' | '>' | '!' | '%' | '@' => quote = true,
            _ => (),
        };
        match v.chars().last().unwrap_or(' ') {
            '}' | ']' | ' ' | ':' => quote = true,
            _ => (),
        };
        quote |= v.chars().all(|c| c.is_digit(10) || c == ':');
        if quote {
            self.char('\'')?;
        }
        self.str(v)?;
        if quote {
            self.char('\'')?;
        }
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> fmt::Result {
        self.indent(Element::Literal)?;
        self.str(unsafe { core::str::from_utf8_unchecked(v) })
    }

    fn serialize_none(self) -> fmt::Result {
        self.indent(Element::Literal)?;
        self.str("null")
    }

    fn serialize_some<T: ser::Serialize + ?Sized>(self, value: &T) -> fmt::Result {
        value.serialize(self)
    }

    fn serialize_unit(self) -> fmt::Result {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> fmt::Result {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> fmt::Result {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> fmt::Result
    where
        T: ser::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ser::Serialize + ?Sized>(
        mut self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> fmt::Result {
        let mut s = SerializeStruct::new(&mut self);
        s.serialize_field(variant, value)?;
        s.end()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, fmt::Error> {
        Ok(SerializeSeq::new(self))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, fmt::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, fmt::Error> {
        unreachable!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, fmt::Error> {
        unreachable!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, fmt::Error> {
        Ok(SerializeMap::new(self))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, fmt::Error> {
        Ok(SerializeStruct::new(self))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, fmt::Error> {
        self.indent(Element::MappingKey)?;
        write!(self.writer, "{}:", variant)?;
        self.push();
        Ok(SerializeStructVariant::new(self))
    }

    fn collect_str<T: fmt::Display + ?Sized>(self, _value: &T) -> fmt::Result {
        unreachable!()
    }
}

/// Create a serializable formatter
pub fn to_fmt<W: fmt::Write, T: ser::Serialize + ?Sized>(w: W, value: &T) -> fmt::Result {
    let mut serializer = Serializer { writer: w, depth: 0, preceding_element: Element::None };
    value.serialize(&mut serializer)
}

#[cfg(test)]
mod tests {
    use serde_derive::Serialize;

    struct Wrapper<T: serde::Serialize>(T);

    impl<T: serde::Serialize> core::fmt::Display for Wrapper<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            crate::to_fmt(f, &self.0)
        }
    }

    #[test]
    fn array() {
        assert_eq!(format!("{}", Wrapper([0, 1, 2])), "- 0\n- 1\n- 2");
    }

    #[test]
    fn bool() {
        assert_eq!(format!("{}", Wrapper(true)), "true");
    }

    #[test]
    fn enum_() {
        #[derive(Serialize)]
        enum Type {
            #[serde(rename = "boolean")]
            Boolean,
            #[serde(rename = "number")]
            Number,
        }

        assert_eq!(format!("{}", Wrapper(Type::Boolean)), "boolean");

        assert_eq!(format!("{}", Wrapper(Type::Number)), "number");
    }

    #[test]
    fn str() {
        assert_eq!(format!("{}", Wrapper("hello")), "hello");
        assert_eq!(format!("{}", Wrapper("")), "''");

        // Characters unescaped if possible
        assert_eq!(format!("{}", Wrapper("ä")), "ä");
        assert_eq!(format!("{}", Wrapper("৬")), "৬");
        assert_eq!(format!("{}", Wrapper("ℝ")), "ℝ"); // 3 byte character
        assert_eq!(format!("{}", Wrapper("💣")), "💣"); // 4 byte character

        assert_eq!(format!("{}", Wrapper("1")), "'1'"); // number
        assert_eq!(format!("{}", Wrapper("1:2")), "'1:2'"); // clock format number
    }

    #[test]
    fn struct_bool() {
        #[derive(Serialize)]
        struct Led {
            led: bool,
        }

        assert_eq!(format!("{}", Wrapper(&Led { led: true })), "led: true");
    }

    #[test]
    fn struct_i8() {
        #[derive(Serialize)]
        struct Temperature {
            temperature: i8,
        }

        assert_eq!(format!("{}", Wrapper(&Temperature { temperature: 127 })), "temperature: 127");
        assert_eq!(format!("{}", Wrapper(&Temperature { temperature: 20 })), "temperature: 20");
        assert_eq!(format!("{}", Wrapper(&Temperature { temperature: -17 })), "temperature: -17");
        assert_eq!(format!("{}", Wrapper(&Temperature { temperature: -128 })), "temperature: -128");
    }

    #[test]
    fn struct_f32() {
        #[derive(Serialize)]
        struct Temperature {
            temperature: f32,
        }

        assert_eq!(
            format!("{}", Wrapper(&Temperature { temperature: -20. })),
            "temperature: -20.0"
        );

        assert_eq!(
            format!("{}", Wrapper(&Temperature { temperature: -20345. })),
            "temperature: -20345.0"
        );

        assert_eq!(
            format!("{}", Wrapper(&Temperature { temperature: -2.3456789012345e-23 })),
            "temperature: -2.3456788e-23"
        );
    }

    #[test]
    fn struct_option() {
        #[derive(Serialize)]
        struct Property<'a> {
            description: Option<&'a str>,
        }

        assert_eq!(
            format!(
                "{}",
                Wrapper(&Property { description: Some("An ambient temperature sensor") })
            ),
            "description: An ambient temperature sensor"
        );

        // XXX Ideally this should produce "{}"
        assert_eq!(format!("{}", Wrapper(&Property { description: None })), "description: null");
    }

    #[test]
    fn struct_u8() {
        #[derive(Serialize)]
        struct Temperature {
            temperature: u8,
        }

        assert_eq!(format!("{}", Wrapper(&Temperature { temperature: 20 })), "temperature: 20");
    }

    #[test]
    fn struct_() {
        #[derive(Serialize)]
        struct Empty {}

        assert_eq!(format!("{}", Wrapper(&Empty {})), r#"{}"#);

        #[derive(Serialize)]
        struct Tuple {
            a: bool,
            b: bool,
        }

        assert_eq!(format!("{}", Wrapper(&Tuple { a: true, b: false })), "a: true\nb: false");
    }

    #[test]
    fn test_unit() {
        let a = ();
        assert_eq!(format!("{}", Wrapper(&a)), "null");
    }

    #[test]
    fn test_newtype_struct() {
        #[derive(Serialize)]
        struct A(pub u32);
        let a = A(54);
        assert_eq!(format!("{}", Wrapper(&a)), "54");
    }

    #[test]
    fn test_newtype_variant() {
        #[derive(Serialize)]
        enum A {
            A(u32),
        }
        let a = A::A(54);

        assert_eq!(format!("{}", Wrapper(&a)), "A: 54");
    }

    #[test]
    fn test_struct_variant() {
        #[derive(Serialize)]
        enum A {
            A { x: u32, y: u16 },
        }
        let a = A::A { x: 54, y: 720 };

        assert_eq!(format!("{}", Wrapper(&a)), "A:\n  x: 54\n  y: 720");
    }

    #[test]
    fn test_serialize_bytes() {
        pub struct SimpleDecimal(f32);

        impl serde::Serialize for SimpleDecimal {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let string = format!("{:.2}", self.0);
                serializer.serialize_bytes(string.as_bytes())
            }
        }

        let sd1 = SimpleDecimal(1.55555);
        assert_eq!(format!("{}", Wrapper(&sd1)), "1.56");

        let sd2 = SimpleDecimal(0.000);
        assert_eq!(format!("{}", Wrapper(&sd2)), "0.00");

        let sd3 = SimpleDecimal(22222.777777);
        assert_eq!(format!("{}", Wrapper(&sd3)), "22222.78");
    }

    #[test]
    fn test_serializable_key() {
        use std::fmt::Write;
        use std::string::String;

        use serde::ser::SerializeMap;

        struct A(usize);

        impl serde::Serialize for A {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut string = String::new();
                write!(string, "A{}", self.0).ok();
                serializer.serialize_str(string.as_str())
            }
        }

        struct B {
            a: A,
            b: usize,
        }

        impl serde::Serialize for B {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(&self.a, &self.b)?;
                map.end()
            }
        }

        assert_eq!(format!("{}", Wrapper(B { a: A(1), b: 2 })), "A1: 2")
    }

    #[test]
    fn test_nested_array() {
        #[derive(Serialize)]
        struct A {
            a: [usize; 2],
        }

        assert_eq!(format!("{}", Wrapper(A { a: [0, 1] })), "a:\n  - 0\n  - 1");

        let array: [[usize; 2]; 2] = [[0, 1], [2, 3]];
        assert_eq!(format!("{}", Wrapper(array)), "- - 0\n  - 1\n- - 2\n  - 3");
    }

    #[test]
    fn test_nested_struct() {
        #[derive(Serialize)]
        struct A {
            v: usize,
        }

        #[derive(Serialize)]
        struct B {
            a: A,
            b: A,
        }

        assert_eq!(
            format!("{}", Wrapper(B { a: A { v: 1 }, b: A { v: 2 } })),
            "a:\n  v: 1\nb:\n  v: 2"
        );
    }
}
