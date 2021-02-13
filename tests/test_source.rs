#![deny(clippy::all, clippy::pedantic)]

use core2::error::Error as StdError;
use core2::io;
use thiserror_core2::Error;

// #[derive(Error, Debug)]
// #[error("implicit source")]
// pub struct ImplicitSource {
//     source: io::Error,
// }

// #[derive(Error, Debug)]
// #[error("explicit source")]
// pub struct ExplicitSource {
//     source: String,
//     #[source]
//     io: io::Error,
// }

#[derive(Error, Debug)]
#[error("boxed source")]
pub struct BoxedSource {
    #[source]
    source: Box<dyn StdError + Send + 'static>,
}

// #[test]
// fn test_implicit_source() {
//     let io = io::Error::new(io::ErrorKind::Other, "oh no!");
//     let error = ImplicitSource { source: io };
//     error.source().unwrap().downcast_ref::<io::Error>().unwrap();
// }

// #[test]
// fn test_explicit_source() {
//     let io = io::Error::new(io::ErrorKind::Other, "oh no!");
//     let error = ExplicitSource {
//         source: String::new(),
//         io,
//     };
//     error.source().unwrap().downcast_ref::<io::Error>().unwrap();
// }

// #[test]
// fn test_boxed_source() {
//     let source = Box::new(io::Error::new(io::ErrorKind::Other, "oh no!"));
//     let error = BoxedSource { source };
//     error.source().unwrap().downcast_ref::<io::Error>().unwrap();
// }
