mod character;
mod env;
mod error;
mod list;
mod macros;
mod map;
mod number;
mod pattern;
mod procedure;
mod string;
mod template;
mod value;
mod vector;

// Data traits ////////////////////////////////////////////////////////////////

pub trait DisplayRep {
    fn to_display(&self) -> String;
}

pub trait ExternalRep {
    fn to_external(&self) -> String;
}

pub trait ValueIterator {
    fn values(&self) -> Box<dyn Iterator<Item = Val>>;
}

// Data Exports ///////////////////////////////////////////////////////////////

// By making them public we can import all data types using
// crate::data::<SomeType> or crate::data::* and we avoid circular imports issues
// this way as all types are used through the lib rather I guess.

pub use character::Char;
pub use env::Env;
pub use error::Error;
pub use list::List;
pub use macros::Macro;
pub use map::Map;
pub use number::Num;
pub use pattern::Pattern;
pub use procedure::{Closure, Formals, Lambda, Procedure, TailCall};
pub use string::Str;
pub use template::Template;
pub use value::Val;
pub use vector::Vector;

use std::rc::Rc;
pub type Environ = Rc<Env<Rc<Str>, Val>>;
