use core::fmt;

use serde::ser;

pub(crate) enum Unreachable {}

impl ser::SerializeTupleStruct for Unreachable {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> fmt::Result {
        unreachable!()
    }

    fn end(self) -> fmt::Result {
        unreachable!()
    }
}

impl ser::SerializeTupleVariant for Unreachable {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> fmt::Result {
        unreachable!()
    }

    fn end(self) -> fmt::Result {
        unreachable!()
    }
}

impl ser::SerializeMap for Unreachable {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> fmt::Result
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> fmt::Result
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> fmt::Result {
        unreachable!()
    }
}
