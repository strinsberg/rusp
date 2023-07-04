use crate::data::{Char, DisplayRep, ExternalRep};
use std::hash::{Hash, Hasher};

// String /////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Str {
    chars: Vec<Char>,
}

impl Str {
    pub fn new(ch: Char, size: usize) -> Str {
        let mut chars = Vec::with_capacity(size);
        for _ in 0..size {
            chars.push(ch.clone());
        }
        Str { chars: chars }
    }

    pub fn fill(&mut self, ch: Char) {
        for c in self.chars.iter_mut() {
            *c = ch.clone();
        }
    }

    // Access //

    pub fn get(&self, idx: usize) -> Option<&Char> {
        if idx < self.chars.len() {
            Some(&self.chars[idx])
        } else {
            None
        }
    }

    pub fn set(&mut self, ch: Char, idx: usize) -> Option<Char> {
        if idx < self.len() {
            let c = self.chars[idx];
            self.chars[idx] = ch.clone();
            Some(c)
        } else {
            None
        }
    }

    pub fn chars(&self) -> std::slice::Iter<'_, Char> {
        self.chars.iter()
    }

    // Information //

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is(&self, other: &str) -> bool {
        // TODO make this more efficient?
        self == &Str::from(other)
    }

    // Functions //

    pub fn append(&self, other: Str) -> Str {
        let mut chars = Vec::with_capacity(self.len() + other.len());
        for ch in self.chars().chain(other.chars()) {
            chars.push(ch.clone())
        }
        Str::from(chars)
    }

    pub fn substring(&self, start: usize, end: usize) -> Str {
        if start >= end {
            return Str::default();
        }
        let last = match end {
            i if i >= self.len() => self.len(),
            i => i,
        };

        let mut chars = Vec::with_capacity(end - start);
        for i in start..last {
            chars.push(self.chars[i].clone());
        }
        Str::from(chars)
    }
}

// Traits /////////////////////////////////////////////////////////////////////

impl Default for Str {
    fn default() -> Str {
        Str::from("")
    }
}

impl From<&str> for Str {
    fn from(s: &str) -> Str {
        Str {
            chars: s.chars().map(|ch| Char::from(ch)).collect(),
        }
    }
}

impl From<&[u8]> for Str {
    fn from(bytes: &[u8]) -> Str {
        Str {
            chars: bytes.iter().map(|b| Char::from(*b)).collect(),
        }
    }
}

impl From<Vec<Char>> for Str {
    fn from(chars: Vec<Char>) -> Str {
        Str {
            chars: chars.iter().map(|ch| ch.clone()).collect(),
        }
    }
}

// Hashing //

impl Hash for Str {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.chars.iter().for_each(|ch| ch.to_byte().hash(state));
    }
}

// Representation //

impl DisplayRep for Str {
    fn to_display(&self) -> String {
        let mut strings = vec![];
        for ch in self.chars.iter() {
            strings.push(match ch.clone() {
                Char::Null => "\0".to_owned(),
                Char::Tab => "\t".to_owned(),
                Char::LineFeed => "\n".to_owned(),
                Char::Space => " ".to_owned(),
                Char::Unsupported => "UNSUP".to_owned(),
                _ => ch.to_display(),
            })
        }
        strings.join("")
    }
}

impl ExternalRep for Str {
    fn to_external(&self) -> String {
        let quote = '"' as u8;
        let mut strings = vec!["\"".to_owned()];
        for ch in self.chars.iter() {
            strings.push(match ch.clone() {
                Char::Null => "\\0".to_owned(),
                Char::Tab => "\\t".to_owned(),
                Char::LineFeed => "\\n".to_owned(),
                Char::Space => " ".to_owned(),
                Char::Unsupported => "UNSUP".to_owned(),
                Char::Char(ch) if ch == quote => "\\\"".to_owned(),
                _ => ch.to_display(),
            })
        }
        strings.push("\"".to_owned());
        strings.join("")
    }
}

impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

impl std::fmt::Debug for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "String{{ {} }}", self.to_external())
    }
}

// Testing ////////////////////////////////////////////////////////////////////

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_new() {
        let string = Str::new(Char::from('a'), 5);
        assert_eq!(string.get(0), Some(Char::from('a')));
        assert_eq!(string.get(1), Some(Char::from('a')));
        assert_eq!(string.get(2), Some(Char::from('a')));
        assert_eq!(string.get(3), Some(Char::from('a')));
        assert_eq!(string.get(4), Some(Char::from('a')));
        assert!(string.get(5).is_none());
    }

    #[test]
    fn test_string_fill() {
        let string = Str::new(Char::from('a'), 5);
        string.fill(Char::from('p'));
        assert_eq!(string.get(0), Some(Char::from('p')));
        assert_eq!(string.get(1), Some(Char::from('p')));
        assert_eq!(string.get(2), Some(Char::from('p')));
        assert_eq!(string.get(3), Some(Char::from('p')));
        assert_eq!(string.get(4), Some(Char::from('p')));
        assert!(string.get(5).is_none());
    }

    #[test]
    fn test_string_length() {
        let string = Str::new(Char::from('a'), 5);
        assert_eq!(string.len(), 5);
    }

    #[test]
    fn test_string_set() {
        let string = Str::from(vec![Char::from('a'); 3]);
        assert_eq!(string.set(Char::from('d'), 0), Some(Char::from('a')));
        assert_eq!(string.set(Char::from('e'), 2), Some(Char::from('a')));
        assert!(string.set(Char::from('l'), 3).is_none());
        assert_eq!(string.get(0), Some(Char::from('d')));
        assert_eq!(string.get(1), Some(Char::from('a')));
        assert_eq!(string.get(2), Some(Char::from('e')));
        assert!(string.get(3).is_none());
    }

    #[test]
    fn test_string_iter() {
        let string = Str::from("hello");
        let mut iter = string.chars();
        assert_eq!(iter.next(), Some(Char::from('h')));
        assert_eq!(iter.next(), Some(Char::from('e')));
        assert_eq!(iter.next(), Some(Char::from('l')));
        assert_eq!(iter.next(), Some(Char::from('l')));
        assert_eq!(iter.next(), Some(Char::from('o')));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_string_representation() {
        let string = Str::from("hello-world\n");
        assert_eq!(string.to_display(), "hello-world\n");
        assert_eq!(string.to_external(), "\"hello-world\\n\"");
    }

    #[test]
    fn test_string_append() {
        let string = Str::from("hello, ");
        let string2 = Str::from("world!");
        let string3 = string.append(string2);
        assert_eq!(string3.to_display(), "hello, world!");
        assert_eq!(string3.len(), 13);
    }

    #[test]
    fn test_string_substring() {
        let string = Str::from("hello");
        assert_eq!(string.substring(0, 5).to_display(), "hello");
        assert_eq!(string.substring(0, 10).to_display(), "hello");
        assert_eq!(string.substring(2, 4).to_display(), "ll");
        assert_eq!(string.substring(4, 1).to_display(), "");
    }
}
*/
