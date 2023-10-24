use crate::data::{DisplayRep, Error, ExternalRep, List, Val};

#[derive(Clone, PartialEq, Eq)]
pub struct Vector {
    mutable: bool,
    vals: Vec<Val>,
}

impl Vector {
    pub fn new(val: Val, size: usize) -> Vector {
        let mut vec = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(val.clone());
        }
        Vector {
            mutable: true,
            vals: vec,
        }
    }

    pub fn tuple(vals: Vec<Val>) -> Vector {
        Vector {
            mutable: false,
            vals,
        }
    }

    pub fn tuple_from_list(ls: List) -> Vector {
        Vector {
            mutable: false,
            vals: ls.values().map(|v| v.clone()).collect(),
        }
    }

    pub fn copy_to_tuple(vec: &Vector) -> Vector {
        Vector {
            mutable: false,
            vals: vec.vals.clone(),
        }
    }

    pub fn copy_to_vec(vec: &Vector) -> Vector {
        Vector {
            mutable: true,
            vals: vec.vals.clone(),
        }
    }

    // Access //

    pub fn get(&self, idx: usize) -> Option<Val> {
        if idx < self.len() {
            Some(self.vals[idx].clone())
        } else {
            None
        }
    }

    pub fn set(&mut self, val: Val, idx: usize) -> Result<(), Error> {
        if !self.mutable {
            Err(Error::Immutable)
        } else if idx >= self.len() {
            Err(Error::OutOfRange)
        } else {
            self.vals[idx] = val.clone();
            Ok(())
        }
    }

    pub fn push(&mut self, val: Val) -> Result<(), Error> {
        if !self.mutable {
            Err(Error::Immutable)
        } else {
            Ok(self.vals.push(val))
        }
    }

    pub fn pop(&mut self) -> Result<(), Error> {
        if !self.mutable {
            Err(Error::Immutable)
        } else {
            self.vals.pop();
            Ok(())
        }
    }

    pub fn freeze(&mut self) {
        self.mutable = false;
    }

    pub fn fill(&mut self, val: Val) -> Result<(), Error> {
        if !self.mutable {
            Err(Error::Immutable)
        } else {
            for v in self.vals.iter_mut() {
                *v = val.clone();
            }
            Ok(())
        }
    }

    pub fn values(&self) -> std::slice::Iter<'_, Val> {
        self.vals.iter()
    }

    // Information //

    pub fn len(&self) -> usize {
        self.vals.len()
    }

    pub fn is_tuple(&self) -> bool {
        !self.mutable
    }
}

// Traits /////////////////////////////////////////////////////////////////////

impl From<Vec<Val>> for Vector {
    fn from(vec: Vec<Val>) -> Vector {
        Vector {
            mutable: true,
            vals: vec,
        }
    }
}

impl From<List> for Vector {
    fn from(ls: List) -> Vector {
        Vector {
            mutable: true,
            vals: ls.values().map(|v| v.clone()).collect(),
        }
    }
}

// Representaitons //
// TODO different representations for tuple vs vector

impl DisplayRep for Vector {
    fn to_display(&self) -> String {
        let vals = self
            .values()
            .map(|v| v.to_display())
            .collect::<Vec<String>>()
            .join(" ");
        if self.mutable {
            format!("[{vals}]",)
        } else {
            format!("#[{vals}]",)
        }
    }
}

impl ExternalRep for Vector {
    fn to_external(&self) -> String {
        let vals = self
            .values()
            .map(|v| v.to_external())
            .collect::<Vec<String>>()
            .join(" ");
        if self.mutable {
            format!("[{vals}]",)
        } else {
            format!("#[{vals}]",)
        }
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vector{{ {} }}", self.to_external())
    }
}

// Testing ////////////////////////////////////////////////////////////////////

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::cell::Cell;

    // Test CellValue //

    #[derive(Clone, Debug, PartialEq)]
    enum TestVal {
        Int(i64),
        Pair(Cell<TestVal>),
    }

    impl CellValue<TestVal> for TestVal {
        fn get_cell(&self) -> Option<Cell<TestVal>> {
            match self {
                TestVal::Int(_) => None,
                TestVal::Pair(cell) => Some(cell.clone()),
            }
        }

        fn is_empty(&self) -> bool {
            false
        }
    }

    impl ExternalRep for TestVal {
        fn to_external(&self) -> String {
            match self {
                TestVal::Int(i) => format!("#{}", i),
                TestVal::Pair(cell) => format!("{}", cell.to_external()),
            }
        }
    }

    impl DisplayRep for TestVal {
        fn to_display(&self) -> String {
            match self {
                TestVal::Int(i) => format!("{}", i),
                TestVal::Pair(cell) => format!("{}", cell.to_display()),
            }
        }
    }

    // Tests //

    #[test]
    fn test_array_new() {
        let arr = Vector::new(TestVal::Int(1), 4);
        assert_eq!(arr.get(0).unwrap().clone(), TestVal::Int(1));
        assert_eq!(arr.get(1).unwrap().clone(), TestVal::Int(1));
        assert_eq!(arr.get(2).unwrap().clone(), TestVal::Int(1));
        assert_eq!(arr.get(3).unwrap().clone(), TestVal::Int(1));
        assert!(arr.get(4).is_none());
    }

    #[test]
    fn test_array_fill() {
        let arr = Vector::new(TestVal::Int(1), 4);
        arr.fill(TestVal::Int(4));
        assert_eq!(arr.get(0).unwrap().clone(), TestVal::Int(4));
        assert_eq!(arr.get(1).unwrap().clone(), TestVal::Int(4));
        assert_eq!(arr.get(2).unwrap().clone(), TestVal::Int(4));
        assert_eq!(arr.get(3).unwrap().clone(), TestVal::Int(4));
        assert!(arr.get(4).is_none());
    }

    #[test]
    fn test_array_length() {
        let arr = Vector::from(vec![TestVal::Int(1), TestVal::Int(2), TestVal::Int(3)]);
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_array_get() {
        let arr = Vector::from(vec![TestVal::Int(1), TestVal::Int(2), TestVal::Int(3)]);
        assert_eq!(arr.get(0).unwrap().clone(), TestVal::Int(1));
        assert_eq!(arr.get(1).unwrap().clone(), TestVal::Int(2));
        assert_eq!(arr.get(2).unwrap().clone(), TestVal::Int(3));
        assert!(arr.get(3).is_none());
    }

    #[test]
    fn test_array_set() {
        let arr = Vector::from(vec![TestVal::Int(1), TestVal::Int(2), TestVal::Int(3)]);
        assert_eq!(
            arr.set(TestVal::Int(88), 0).unwrap().clone(),
            TestVal::Int(1)
        );
        assert_eq!(
            arr.set(TestVal::Int(99), 1).unwrap().clone(),
            TestVal::Int(2)
        );
        assert!(arr.set(TestVal::Int(100), 3).is_none());
        assert_eq!(arr.get(0).unwrap().clone(), TestVal::Int(88));
        assert_eq!(arr.get(1).unwrap().clone(), TestVal::Int(99));
        assert_eq!(arr.get(2).unwrap().clone(), TestVal::Int(3));
    }

    #[test]
    fn test_array_iter() {
        let arr = Vector::from(vec![TestVal::Int(1), TestVal::Int(2), TestVal::Int(3)]);
        let mut iter = arr.values();
        assert_eq!(iter.next(), Some(TestVal::Int(1)));
        assert_eq!(iter.next(), Some(TestVal::Int(2)));
        assert_eq!(iter.next(), Some(TestVal::Int(3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_array_from_list() {
        let cell = Cell::new(TestVal::Int(5), None);
        let cell2 = Cell::new(TestVal::Int(4), Some(TestVal::Pair(cell.clone())));
        let cell3 = Cell::new(TestVal::Int(3), Some(TestVal::Pair(cell2.clone())));
        let cell4 = Cell::new(TestVal::Int(2), Some(TestVal::Pair(cell3.clone())));
        let cell5 = Cell::new(TestVal::Int(1), Some(TestVal::Pair(cell4.clone())));

        let arr = Vector::from(TestVal::Pair(cell5));
        let mut iter = arr.values();
        assert_eq!(iter.next(), Some(TestVal::Int(1)));
        assert_eq!(iter.next(), Some(TestVal::Int(2)));
        assert_eq!(iter.next(), Some(TestVal::Int(3)));
        assert_eq!(iter.next(), Some(TestVal::Int(4)));
        assert_eq!(iter.next(), Some(TestVal::Int(5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_array_display() {
        let arr = Vector::from(vec![TestVal::Int(1), TestVal::Int(2), TestVal::Int(3)]);
        assert_eq!(arr.to_display(), "#(1 2 3)".to_owned());
        assert_eq!(arr.to_external(), "#(#1 #2 #3)".to_owned());
    }
}
*/
