use serde::Serialize;
pub use svd_macros::peripheral;

type Str = &'static str;
type List<T> = &'static [T];

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
    pub fields: List<Field>,
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
