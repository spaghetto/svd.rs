use svd::fields;
use svd::peripheral;
use svd::Fields;
use svd::Peripheral;

const A: Fields = fields! {
    0..12 => RSTRX: "Reset Receiver",
};

const F: Peripheral = peripheral! {
    0xFFFFF200 => dbgu: "Debug Unit" {
        0x0000 => cr: "Control Register" {
            ..A,
            0..12 => RSTRX: "Reset Receiver",
        }
    }
};

fn main() {
    println!("{}", serde_json::to_string_pretty(&F).unwrap())
}
