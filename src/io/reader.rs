use crate::data::{Error, List, Map, Val, Vector};
use crate::io::scanner::{Scanner, Token};
use std::rc::Rc;

// TODO confirm that the reader identifies all proper stopping points when
// parsing complex forms.

#[derive(Debug)]
pub struct StringReader {
    scanner: Scanner,
}

impl StringReader {
    pub fn new(string: &str) -> StringReader {
        StringReader {
            scanner: Scanner::new(string),
        }
    }

    pub fn read(&mut self) -> Result<Val, Error> {
        let next = self.scanner.next()?;
        self.read_helper(next)
    }

    pub fn read_forms(&mut self) -> Result<Vec<Val>, Error> {
        let mut forms = Vec::new();
        loop {
            let next = self.scanner.next()?;
            if next == Token::EOF {
                break;
            } else {
                forms.push(self.read_helper(next)?)
            }
        }
        Ok(forms)
    }

    fn read_helper(&mut self, token: Token) -> Result<Val, Error> {
        match token {
            Token::Identifier(s) => Ok(Val::Symbol(Rc::new(s))),
            Token::Keyword(s) => Ok(Val::Keyword(Rc::new(s))),
            Token::Boolean(b) => Ok(Val::Bool(b)),
            Token::Number(num) => Ok(Val::Number(num)),
            Token::Character(ch) => Ok(Val::Char(ch)),
            Token::String(s) => Ok(Val::from(s)),
            Token::LParen => self.read_list(),
            Token::ListOpen => self.read_list_literal(),
            Token::VecOpen => self.read_vector_val(),
            Token::TupleOpen => self.read_tuple_val(),
            Token::MapOpen => self.read_map("table"),
            Token::DictOpen => self.read_map("dict"),
            Token::Deref => self.read_deref(),
            Token::None => Ok(Val::None),
            tk => Err(Error::BadToken(self.scanner.line, tk.to_string())),
        }
    }

    fn read_list(&mut self) -> Result<Val, Error> {
        // ( was used by caller
        let mut vec = Vec::new();
        let mut val = self.scanner.next()?;

        while val != Token::RParen {
            vec.push(self.read_helper(val)?);
            val = self.scanner.next()?;
        }
        Ok(Val::list_from_vec(&vec))
    }

    fn read_list_literal(&mut self) -> Result<Val, Error> {
        // #( was used by caller
        let mut vec = Vec::new();
        let mut val = self.scanner.next()?;

        while val != Token::RParen {
            vec.push(self.read_helper(val)?);
            val = self.scanner.next()?;
        }
        // is syntactic sugar for (list ...)
        match List::from_vec(&vec) {
            Some(ls) => Ok(Val::from(List::new(Val::symbol("list"), Some(Rc::new(ls))))),
            None => Ok(Val::Empty),
        }
    }

    fn read_vector_val(&mut self) -> Result<Val, Error> {
        let vec = self.read_vector()?;
        Ok(Val::from(vec))
    }

    fn read_tuple_val(&mut self) -> Result<Val, Error> {
        // #[ was used by caller
        let vec = self.read_vector()?;
        Ok(Val::from(Vector::tuple(vec)))
    }

    fn read_vector(&mut self) -> Result<Vec<Val>, Error> {
        // [ was used by caller
        let mut vec = Vec::new();
        let mut val = self.scanner.next()?;

        while val != Token::VecClose {
            vec.push(self.read_helper(val)?);
            val = self.scanner.next()?;
        }
        Ok(vec)
    }

    fn read_map(&mut self, kind: &str) -> Result<Val, Error> {
        // { or #{ was used by the caller
        let mut vals = vec![];
        loop {
            let val = self.scanner.next()?;
            if val == Token::MapClose {
                break;
            } else {
                vals.push(self.read_helper(val)?);
            }
        }

        if vals.len() == 0 {
            let mut m = Map::new();
            if kind == "dict" {
                m.freeze()
            }
            Ok(Val::from(m))
        } else {
            Ok(Val::from(List::new(
                Val::symbol(kind),
                Some(Rc::new(
                    List::from_vec(&vals).expect("vec should be covertable to list"),
                )),
            )))
        }
    }

    // TODO could use this as a reference for how to make [] etc syntactic
    // sugar instead of separate forms. I.e. so that when we read and evaluate
    // them we do not need to have a separate eval rule for collection
    // evaluation for each collection.
    fn read_deref(&mut self) -> Result<Val, Error> {
        // @ was used by caller
        let ident = self.scanner.next()?;
        // is syntactic sugar for (deref identifier)
        match ident {
            Token::Identifier(id) => Ok(Val::list_from_vec(&vec![
                Val::symbol("deref"),
                Val::Symbol(Rc::new(id)),
            ])),
            _ => Err(Error::DerefNotIdent(ident.to_string())),
        }
    }
}

// Tests //////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_values_parsed_by_the_scanner() {
        assert_eq!(StringReader::new("10").read(), Ok(Val::from(10)));
        assert_eq!(StringReader::new("1.234").read(), Ok(Val::from(1.234)));
        assert_eq!(StringReader::new("#d1.234").read(), Ok(Val::from(1.234)));
        assert_eq!(StringReader::new("#o11").read(), Ok(Val::from(9)));
        assert_eq!(StringReader::new("#b11").read(), Ok(Val::from(3)));
        assert_eq!(StringReader::new("#x1F").read(), Ok(Val::from(31)));
        assert_eq!(StringReader::new("#true").read(), Ok(Val::Bool(true)));
        assert_eq!(StringReader::new("#f").read(), Ok(Val::Bool(false)));
        assert_eq!(StringReader::new("\\H").read(), Ok(Val::from('H')));
        assert_eq!(StringReader::new("\\space").read(), Ok(Val::from(' ')));
        assert_eq!(StringReader::new("\\slash").read(), Ok(Val::from('\\')));
        assert_eq!(
            StringReader::new("hello-world!").read(),
            Ok(Val::symbol("hello-world!"))
        );
    }

    #[test]
    fn test_reading_strings() {
        assert_eq!(
            StringReader::new("\"hello, world!\"").read().unwrap(),
            Val::from("hello, world!"),
        );
    }

    #[test]
    fn test_reading_lists() {
        let expr = "#(1 2 3 4)";
        let expect = "(list 1 2 3 4)";
        let result = StringReader::new(expr).read().unwrap().to_string();
        assert_eq!(result, expect);

        let expr = "#()";
        let expect = "#()";
        let result = StringReader::new(expr).read().unwrap().to_string();
        assert_eq!(result, expect);

        let expr = "#(1 (+ 2 3) 4)";
        let expect = "(list 1 (+ 2 3) 4)";
        let result = StringReader::new(expr).read().unwrap().to_string();
        assert_eq!(result, expect);
    }

    #[test]
    fn test_reading_vectors() {
        let expr = "[1 2 3 4]";
        let result = StringReader::new(expr).read().unwrap().to_string();
        assert_eq!(result, expr);

        let expr = "#[1 2 3 4]";
        let result = StringReader::new(expr).read().unwrap().to_string();
        assert_eq!(result, expr);
    }

    #[test]
    fn test_reading_maps() {
        // This testing method only allows us to use 1 entry as it is an unordered map
        let expr = "{:a 2 :b 3}";
        let expect = "(table :a 2 :b 3)";
        let result = StringReader::new(expr).read().unwrap().to_string();
        assert_eq!(result, expect);

        let expr = "#{:a 2 :b 3}";
        let expect = "(dict :a 2 :b 3)";
        let result = StringReader::new(expr).read().unwrap().to_string();
        assert_eq!(result, expect);
    }

    #[test]
    fn test_reading_several_forms() {
        let text = "1 #true\n\n(define a 5)\n#none";
        let result: Vec<String> = StringReader::new(text)
            .read_forms()
            .unwrap()
            .into_iter()
            .map(|val| val.to_string())
            .collect();

        assert_eq!(result[0], "1".to_string());
        assert_eq!(result[1], "#t".to_string());
        assert_eq!(result[2], "(define a 5)".to_string());
        assert_eq!(result[3], "#none".to_string());
    }
}
