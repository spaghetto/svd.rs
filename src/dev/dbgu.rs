use svd_macros::svd;

// macro_rules! register {
//     ( $($k:ident = $v:expr),*; fields = [ $($off:literal => $name:ident: $desc:expr),*, ] ) => {
//         Register {
//             $($k: $v.into()),*,
//             fields: vec! [ $(Field {
//                 name: stringify!($name).into(),
//                 desc: $desc.into(),
//                 bitOffset: $off,
//                 bitWidth: 1,
//             }),* ]
//         }
//     }
// }

// macro_rules! peripheral {
//     // (register:ident) => {};
//     // ($k:ident $($tail:tt)*) => { foo = "bar" };

//     // ( $($k:ident = $v:expr),*, $(register $reg:expr)*) => {
        
//     // };
//     ( $($k:ident = $v:expr),*; $(register $reg:literal { $($body:tt)* }),*  ) => {
//         Peripheral {
//             $($k: $v.into()),*,
//             regs: vec! [
//                 $(
//                 register! {
//                     name = $reg,
//                     $($body)*
//                 }),*
//             ]
//         }
//     };
// }

// const DEV: Peripheral = svd! {
//     peripheral "dbgu" {
//         name: "dbgu",
//         desc: "Debug Unit",
//         addr: 0xFFFFF200,

//         register "cr" {
//             desc: "Control Register",
//             addr: 0x0000,

//             fields: [
//                 2 => RSTRX: "Reset Receiver",
//             ],
//         },
//     }
// };


