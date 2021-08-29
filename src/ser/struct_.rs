use core::fmt;

use serde::ser;

use crate::ser::{Element, Serializer};

pub struct SerializeStruct<'a, W: fmt::Write> {
    serializer: &'a mut Serializer<W>,
    empty: bool,
}

impl<'a, W: fmt::Write> SerializeStruct<'a, W> {
    pub(crate) fn new(serializer: &'a mut Serializer<W>) -> Self {
        Self { serializer, empty: true }
    }
}

impl<'a, W: fmt::Write> ser::SerializeStruct for SerializeStruct<'a, W> {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> fmt::Result
    where
        T: ser::Serialize + ?Sized,
    {
        if !self.empty {
            self.serializer.char('\n')?;
        }
        self.empty = false;
        self.serializer.indent(Element::MappingKey)?;
        write!(self.serializer.writer, "{}:", key)?;
        self.serializer.push();
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

pub struct SerializeStructVariant<'a, W: fmt::Write> {
    serializer: &'a mut Serializer<W>,
    empty: bool,
}

impl<'a, W: fmt::Write> SerializeStructVariant<'a, W> {
    pub(crate) fn new(serializer: &'a mut Serializer<W>) -> Self {
        Self { serializer, empty: true }
    }
}

impl<'a, W: fmt::Write> ser::SerializeStructVariant for SerializeStructVariant<'a, W> {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> fmt::Result
    where
        T: ser::Serialize + ?Sized,
    {
        if !self.empty {
            self.serializer.char('\n')?;
        }
        self.empty = false;
        self.serializer.indent(Element::MappingKey)?;
        write!(self.serializer.writer, "{}:", key)?;
        self.serializer.push();
        value.serialize(&mut *self.serializer)?;
        self.serializer.pop();
        Ok(())
    }

    fn end(self) -> fmt::Result {
        if self.empty {
            self.serializer.str("{}")?;
        }
        self.serializer.pop();
        Ok(())
    }
}
