use crate::data::Val;

// TODO The error type needs to be remade because it accidentally got deleted.
// TODO I think if these are runtime errors then the scanner and reader errors
// need to be combined with the base error type. Either that or we need to think
// about what parts of the program will return Err(Error) and which parts will just
// return a value wich will be of the rups type Error. I.e. in the interpreter
// maybe it is a fatal error if code that is passed in cannot be parsed, but maybe
// if we use a read function to get input from the user the parse error is a
// runtime error that the user could catch. So some thought needs to go into what
// is an error that should be dealt with by me in rust and what should be dealt
// with by the user/interpreter.
// TODO I think regarding the above the issue is not what is and isn't an Error, but
// how we use them in functions. A function that we want to deal with errors in rust
// can return an Err(Error) and be dealt with by me. A function that could error at
// runtime for the user to handle will just return Val and errors can be returned
// from those functions as Val::Error(Error). So, we can decide to combine the
// errors together or not, though if not we need an Error::ScanErr variant.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    OutOfRange,
    OutOfRangeIdx(&'static str, usize, Val),
    Undeclared(String),
    BadIndex(usize, Val),
    ArgsNotList,
    Arity(&'static str),
    BadArg(usize),
    BadType(Val),
    ArgType(&'static str, &'static str, Val),
    DivideByZero,
    // TODO replace with a scan error
    CantParseNum(String),
    NotAProcedure(Val),
    NotHashable(Val),
    Immutable,
    Eof(usize),
    BadChar(usize, char),
    BadToken(usize, String),
    BadIdentifier(usize, String),
    BadEscape(usize, String),
    BadNumber(usize, String),
    MultiLineString(usize),
    OddMapPairs(usize),
    BadRule(Val),
    BadPattern(Val),
    BadTemplate(Val),
    TemplateElipseNotVar,
    NoMacroMatch(String),
    Throw(Val, Val, Vec<Val>),
}

// Scan/Read Error ////////////////////////////////////////////////////////////

/*
#[derive(Debug, Clone, PartialEq)]
pub enum ScanError {
    Eof(usize),
    BadChar(usize, char),
    BadToken(usize, String),
    BadIdentifier(usize, String),
    BadEscape(usize, String),
    BadNumber(usize, String),
    MultiLineString(usize),
    OddMapPairs(usize),
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScanError::Eof(line) => {
                write!(f, "ReadError: Line: {line}, unexpected EOF")
            }
            ScanError::BadToken(line, tk) => {
                write!(f, "ReadError: Line: {line}, unexpected token: {tk}")
            }
            ScanError::BadChar(line, ch) => {
                write!(f, "ReadError: Line: {line}, unexpected character: {ch}")
            }
            ScanError::BadIdentifier(line, id) => {
                write!(f, "ReadError: Line: {line}, invalid identifier: {id}")
            }
            ScanError::BadEscape(line, s) => {
                write!(f, "ReadError: Line: {line}, invalid escape character: {s}")
            }
            ScanError::BadNumber(line, n) => {
                write!(f, "ReadError: Line: {line}, invalid number: {n}")
            }
            ScanError::MultiLineString(line) => {
                write!(
                    f,
                    "ReadError: Line: {line}, string literals cannot span multiple lines"
                )
            }
        }
    }
}
*/

// UserError //////////////////////////////////////////////////////////////////

/*
#[derive(Debug, Clone, PartialEq)]
pub enum UserError {
    Undeclared(Str),
    ArgType(String, String, Val),
    OutOfRange(usize, Val),
    IndexError(String, usize, Val),
    Arity(String),
    Syntax(Val),
    ReadError(ScanError),
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserError::Undeclared(name) => {
                write!(f, "Error: undeclared symbol: {name}")
            }
            UserError::Arity(name) => {
                write!(f, "Error in {name}: incorrect argument count")
            }
            UserError::ArgType(name, kind, expr) => {
                write!(f, "Error in {name}: {expr} must be {kind}")
            }
            UserError::OutOfRange(idx, val) => {
                write!(f, "Error: {idx} is not a valid index for {val}")
            }
            UserError::IndexError(name, idx, val) => {
                write!(f, "Error in {name}: {idx} is not a valid index for {val}")
            }
            UserError::Syntax(expr) => {
                write!(f, "Error: invalid syntax: {expr}")
            }
            UserError::ReadError(err) => {
                write!(f, "{}", err)
            }
        }
    }
}
*/
