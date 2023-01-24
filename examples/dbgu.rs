use svd::Peripheral;
use svd::peripheral;

const F: Peripheral = peripheral! {
    0xFFFFF200 => dbgu: "Debug Unit" {
        0x0000 => cr: "Control Register" {
            1 => RSTRX: "Reset Receiver",
        }
    }
};

fn main() {
    println!("{}", serde_json::to_string_pretty(&F).unwrap())
}
