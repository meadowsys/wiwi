use crate::rand::ThreadLocalChaCha20Rng as RngSecure;

mod error;
pub mod prim;

pub use error::{ Error, Result };
