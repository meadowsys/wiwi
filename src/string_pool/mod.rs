pub mod pool;
pub mod string;

#[doc(inline)]
pub use self::string::String;
#[doc(inline)]
pub use self::pool::{ Pool, GlobalPool };
