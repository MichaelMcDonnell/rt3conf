//! The `rt3conf` crate consists of the binary and library with the same name.
//! The library is not intended to be used on its own, only together with binary.
//!
//! The [Engine](crate::Engine) struct contains all of the settings that
//! are stored in the `engine.cfg` configuration file. The struct is created
//! the [new](crate::Engine::new) method. It can be written to a file by first
//! calling the [serialize](crate::Engine::serialize) method to covert it
//! into a vector of bytes (i.e. `Vec<u8>`) and then saving the bytes to a file.
mod engine;
pub use engine::Engine;