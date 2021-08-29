use core::fmt;

use serde::ser;

use crate::ser::{Element, Serializer};

pub struct SerializeSeq<'a, W: fmt::Write> {
    serializer: &'a mut Serializer<W>,
    empty: bool,
}

impl<'a, W: fmt::Write> SerializeSeq<'a, W> {
    pub(crate) fn new(serializer: &'a mut Serializer<W>) -> Self {
        Self { serializer, empty: true }
    }
}

impl<'a, W: fmt::Write> ser::SerializeSeq for SerializeSeq<'a, W> {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_element<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> fmt::Result {
        if !self.empty {
            self.serializer.char('\n')?;
        }
        self.empty = false;
        self.serializer.indent(Element::SequenceEntry)?;
        self.serializer.str("-")?;
        self.serializer.push();
        value.serialize(&mut *self.serializer)?;
        self.serializer.pop();
        Ok(())
    }

    fn end(self) -> fmt::Result {
        if self.empty {
            self.serializer.str("[]")?;
        }
        Ok(())
    }
}

impl<'a, W: fmt::Write> ser::SerializeTuple for SerializeSeq<'a, W> {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_element<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> fmt::Result {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> fmt::Result {
        ser::SerializeSeq::end(self)
    }
}
