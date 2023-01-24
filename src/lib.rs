use serde::{ser::SerializeSeq, Serialize};
use std::slice::Iter;
pub use svd_macros::{fields, peripheral};

type Str = &'static str;
type List<T> = &'static [T];

#[derive(Debug, Serialize)]
pub struct Device {
    pub vendor: Str,
    pub name: Str,
    #[serde(rename = "description")]
    pub desc: Str,
    pub version: usize,

    #[serde(rename = "addressUnitBits")]
    pub bits: usize,
    pub width: usize,

    pub peripherals: &'static [Peripheral],
}

#[derive(Debug, Serialize)]
pub struct Peripheral {
    pub name: Str,
    #[serde(rename = "description")]
    pub desc: Str,
    #[serde(rename = "baseAddress")]
    pub addr: usize,
    #[serde(rename = "registers")]
    pub regs: List<Register>,
}

#[derive(Debug, Serialize)]
pub struct Register {
    pub name: Str,
    #[serde(rename = "description")]
    pub desc: Str,
    #[serde(rename = "addressOffset")]
    pub addr: usize,
    pub fields: Fields,
}

#[derive(Debug, Serialize)]
pub struct Field {
    pub name: Str,
    #[serde(rename = "description")]
    pub desc: Str,
    #[serde(rename = "bitOffset")]
    pub bit_offset: usize,
    #[serde(rename = "bitWidth")]
    pub bit_width: usize,
}

#[derive(Debug)]
pub struct Fields {
    pub fields: List<Field>,
    pub base: Option<&'static Fields>,
}

impl<'a> IntoIterator for &'a Fields {
    type Item = &'a Field;
    type IntoIter = FieldsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FieldsIter {
            fields: self.fields.iter(),
            base: self.base.map(|b| Box::new(b.into_iter())),
        }
    }
}

pub struct FieldsIter<'a> {
    fields: Iter<'a, Field>,
    base: Option<Box<FieldsIter<'a>>>,
}

impl<'a> Iterator for FieldsIter<'a> {
    type Item = &'a Field;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(base) = &mut self.base {
            return base.next().or_else(|| self.fields.next());
        }
        self.fields.next()
    }
}

impl<'a> ExactSizeIterator for FieldsIter<'a> {
    fn len(&self) -> usize {
        self.fields.len() + self.base.as_ref().map(|i| i.len()).unwrap_or(0)
    }
}

impl Serialize for Fields {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let it = self.into_iter();
        let mut seq = serializer.serialize_seq(Some(it.len()))?;
        for element in it {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}
