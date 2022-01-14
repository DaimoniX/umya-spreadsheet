// c:majorTickMark
use super::TickMarkValues;
use super::super::super::EnumValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct MajorTickMark {
    val: EnumValue<TickMarkValues>,
}
impl MajorTickMark {
    pub fn get_val(&self)-> &TickMarkValues {
        &self.val.get_value()
    }

    pub fn set_val(&mut self, value:TickMarkValues)-> &mut MajorTickMark {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader:&mut Reader<R>,
        e:&BytesStart
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:majorTickMark
        write_start_tag(writer, "c:majorTickMark", vec![
            ("val", &self.val.get_value_string()),
        ], true);
    }
}
