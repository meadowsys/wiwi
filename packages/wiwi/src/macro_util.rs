extern crate wiwiwiwiwi;
extern crate wiwiwiwiwiwiwiwiwiwi;

pub use wiwiwiwiwiwiwiwiwiwi::macro_recurse;
pub use wiwiwiwiwiwiwiwiwiwi::void;

/// Executes code with cloned values, so the executed code
/// can take ownership of the value without moving the original
///
/// Read on if you want to know why I wrote this smol macro and what problems
/// it's designed to fix, but for those just wanting to use the thing:
///
/// ```
/// # use wiwi::macro_util::with_cloned;
/// let s1 = String::from("a string");
/// let s2 = String::from("another string");
///
/// let new = with_cloned! { mut s1, s2 in
///    // you may specify one or more variables to be cloned in a comma-seperated list
///    // s1 and s2 are cloned in here, and you can optionally specify `mut`
///
///    // do something with the cloned values, using any arbitrary code...
///    let mut joined = s2; // moves `s2`
///    joined.push_str(&s1);
///    // demonstrate that it is indeed a clone
///    drop::<String>(s1); // moves `s1`
///
///    // you can also "return" values out of the macro
///    joined
/// };
///
/// // originals are not modified or moved out,
/// // despite the bindings within the macro being moved
/// assert_eq!(s1, "a string");
/// assert_eq!(s2, "another string");
///
/// // returned value from macro
/// assert_eq!(new, "another stringa string");
/// ```
///
/// Now onto ramble. Let's say you have some code...
///
/// ```
/// # use std::sync::{ Arc, Mutex };
/// # use std::thread;
/// let shared_data = Arc::new(Mutex::new(42));
///
/// // send a copy to another thread to concurrently mutate...
/// let _shared_data = shared_data.clone();
/// let join_handle = thread::spawn(move || {
///    // do something with this data...
///    *_shared_data.lock().unwrap() *= 2;
/// });
///
/// // ... while keeping one to use
/// *shared_data.lock().unwrap() *= 5;
///
/// join_handle.join().unwrap();
//  ...wait, this was not intentional lol
/// assert_eq!(*shared_data.lock().unwrap(), 420);
/// ```
///
/// ... this is not great, since you'd have to use `_shared_data`, which is
/// different from the original one, and the underscore might look kind of ugly,
/// and it's just not ideal.
///
/// It can also be done with a temporary scope:
///
/// ```
/// # use std::sync::{ Arc, Mutex };
/// # use std::thread;
/// # let shared_data = Arc::new(Mutex::new(42));
/// let join_handle = {
///    let shared_data = shared_data.clone();
///    thread::spawn(move || {
///       *shared_data.lock().unwrap() *= 2;
///    })
/// };
///
/// *shared_data.lock().unwrap() *= 5;
///
/// join_handle.join().unwrap();
/// assert_eq!(*shared_data.lock().unwrap(), 420);
/// ```
///
/// ... not really what we would do (really only minor style preferences, nothing
/// actually relating to function), but still has some boilerplate, and not the
/// most optimal either way.
///
/// In our short(...ish?) time writing rust code, I've already noticed I use this
/// pattern a lot when dealing with shared state across threads. There are likely
/// patterns that we don't know about, that would do something similar here (clone
/// a value, then consume the clone, then access original). Or maybe we do know
/// about them, but we haven't thought about it at time of writing this documentation
/// :p. Needing to write all that boilerplate code for an otherwise very simple
/// operation, doesn't feel very ergonomic or elegant, and quickly gets repetitive.
///
/// This macro can help with that:
///
/// ```
/// # use std::sync::{ Arc, Mutex };
/// # use std::thread;
/// # use wiwi::macro_util::with_cloned;
/// # let shared_data = Arc::new(Mutex::new(42));
/// let join_handle = with_cloned! { shared_data in
///    // `shared_data` in here will refer to a clone, shadowing the original,
///    // and I can do whatever, including taking ownership of it
///    thread::spawn(move || {
///       *shared_data.lock().unwrap() *= 2;
///    })
/// };
///
/// // original is not touched, this is no problem
/// *shared_data.lock().unwrap() *= 5;
///
/// join_handle.join().unwrap();
/// assert_eq!(*shared_data.lock().unwrap(), 420);
/// ```
///
/// It cut out most of the boilerplate, and just feels a lot nicer to use.
///
/// The syntax of the macro is, first a (comma seperated) list of the variables
/// that should have a cloned version available, followed by keyword `in`, then
/// any statements you would like to run. Essentially, you can think of the stuff
/// after `in` as just a regular block. Those who know what [swift closures] look
/// like may recognise this syntax hehe
///
/// [swift closures]: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/closures#Closure-Expression-Syntax
///
/// You can also bind the cloned values mutably, as well as return values out
/// of the macro:
///
/// ```
/// # use wiwi::macro_util::with_cloned;
/// let string = String::new();
///
/// let modified_string = with_cloned! { mut string in
///    //                                ^^^
///
///    // do whatever
///    string.push_str("uwu");
///
///    assert_eq!(string, "uwu");
///    string
/// };
///
/// // original is untounched
/// assert_eq!(string, "");
///
/// // returned the modified version from within the macro
/// assert_eq!(modified_string, "uwu");
/// ```
///
/// Ideally you could bind variables mutably or not on a per variable basis,
/// but unfortunately, I can't seem to find a way to do so elegantly >~< so it's
/// all or nothing for now.
///
/// Random, but the below code snippets are equivalent:
///
/// ```
/// # use wiwi::macro_util::with_cloned;
/// # let value = String::new();
/// let cloned1 = value.clone();
/// // macro clones it for the inner code, but the inner code
/// // just returns it... so it is just a clone
/// let cloned2 = with_cloned! { value in value };
///
/// assert_eq!(value, cloned1);
/// assert_eq!(value, cloned2);
/// ```
#[doc(inline)]
pub use wiwiwiwiwiwiwiwiwiwi::with_cloned;

pub use wiwiwiwiwiwiwiwiwiwi::with_cloned_2;
pub use wiwiwiwiwiwiwiwiwiwi::with_cloned_3;
pub use wiwiwiwiwiwiwiwiwiwi::with_cloned_4;
pub use wiwiwiwiwi::with_cloned as with_cloned_proc;
