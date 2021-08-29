use core::fmt;

use serde::ser;

use crate::ser::{Element, Serializer};

pub struct SerializeMap<'a, W: fmt::Write> {
    serializer: &'a mut Serializer<W>,
    empty: bool,
}

impl<'a, W: fmt::Write> SerializeMap<'a, W> {
    pub(crate) fn new(serializer: &'a mut Serializer<W>) -> Self {
        Self { serializer, empty: true }
    }
}

impl<'a, W: fmt::Write> ser::SerializeMap for SerializeMap<'a, W> {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_key<T: ser::Serialize + ?Sized>(&mut self, key: &T) -> fmt::Result {
        if !self.empty {
            self.serializer.char('\n')?;
        }
        self.empty = false;
        self.serializer.indent(Element::PreMappingKey)?;
        key.serialize(&mut *self.serializer)?;
        self.serializer.push();
        self.serializer.str(":")
    }

    fn serialize_value<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> fmt::Result {
        value.serialize(&mut *self.serializer)?;
        self.serializer.pop();
        Ok(())
    }

    fn end(self) -> fmt::Result {
        if self.empty {
            self.serializer.str("{}")?;
        }
        Ok(())
    }
}
