use crate::data::{DisplayRep, ExternalRep, Val};
use std::rc::Rc;

// List ///////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct List {
    head: Val,
    tail: Option<Rc<List>>,
    length: usize,
}

impl List {
    // Constructors //

    pub fn new(head: Val, tail: Option<Rc<List>>) -> List {
        let len = match tail.clone() {
            Some(ls) => ls.length + 1,
            None => 1,
        };
        List {
            head: head,
            tail: tail,
            length: len,
        }
    }

    pub fn from_vec(vec: &[Val]) -> Option<List> {
        List::from_vec_with_tail(vec, None)
    }

    pub fn from_vec_with_tail(vec: &[Val], tail: Option<Rc<List>>) -> Option<List> {
        if vec.len() == 0 {
            return None;
        }

        let mut list = List::new(vec[vec.len() - 1].clone(), tail);
        for v in vec[..vec.len() - 1].iter().rev() {
            list = List::new(v.clone(), Some(Rc::new(list)));
        }
        Some(list)
    }

    pub fn cons(a: Val, b: Val) -> List {
        match b {
            Val::Empty => List::new(a, None),
            Val::List(ls) => List::new(a, Some(ls)),
            _ => List::new(a, Some(Rc::new(List::new(b, None)))),
        }
    }

    // Access //

    pub fn head(&self) -> &Val {
        &self.head
    }

    pub fn tail(&self) -> Option<Rc<List>> {
        self.tail.clone()
    }

    pub fn get(&self, idx: usize) -> Option<Val> {
        for (i, v) in self.values().enumerate() {
            if i == idx {
                return Some(v);
            }
        }
        None
    }

    pub fn first(&self) -> Option<Val> {
        Some(self.head.clone())
    }

    pub fn second(&self) -> Option<Val> {
        self.get(1)
    }

    pub fn third(&self) -> Option<Val> {
        self.get(2)
    }

    pub fn fourth(&self) -> Option<Val> {
        self.get(3)
    }

    pub fn fifth(&self) -> Option<Val> {
        self.get(4)
    }

    // Iterator //

    pub fn values(&self) -> ListValIter {
        ListValIter::new(self.clone())
    }

    // Information //

    pub fn len(&self) -> usize {
        self.length
    }
}

// Traits /////////////////////////////////////////////////////////////////////

impl DisplayRep for List {
    fn to_display(&self) -> String {
        let strings: Vec<String> = self.values().map(|v| v.to_display()).collect();
        format!("({})", strings.join(" "))
    }
}

impl ExternalRep for List {
    fn to_external(&self) -> String {
        let strings: Vec<String> = self.values().map(|v| v.to_external()).collect();
        format!("({})", strings.join(" "))
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

impl std::fmt::Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "List{{ {} }}", self.to_external())
    }
}

// ListValue Iterator /////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct ListValIter {
    list: Option<Rc<List>>,
}

impl ListValIter {
    pub fn new(list: List) -> ListValIter {
        ListValIter {
            list: Some(Rc::new(list)),
        }
    }
}

impl Iterator for ListValIter {
    type Item = Val;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            Some(ref list) => {
                let next = match list.tail() {
                    Some(tail) => Some(tail.clone()),
                    None => None,
                };
                let current = Some(list.head().clone());
                self.list = next;
                current
            }
            None => None,
        }
    }
}

// Testing ////////////////////////////////////////////////////////////////////
/*
#[cfg(test)]
mod tests {
    use super::*;

    // Test ListValue //

    #[derive(Clone, Debug, PartialEq)]
    enum TestVal {
        Int(i64),
        Pair(List<TestVal>),
    }

    impl ListValue<TestVal> for TestVal {
        fn get_cell(&self) -> Option<List<TestVal>> {
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

    // Helpers //

    fn make_list_5() -> List<TestVal> {
        let cell = List::new(TestVal::Int(5), None);
        let cell2 = List::new(TestVal::Int(4), Some(TestVal::Pair(cell.clone())));
        let cell3 = List::new(TestVal::Int(3), Some(TestVal::Pair(cell2.clone())));
        let cell4 = List::new(TestVal::Int(2), Some(TestVal::Pair(cell3.clone())));
        let cell5 = List::new(TestVal::Int(1), Some(TestVal::Pair(cell4.clone())));
        cell5
    }

    fn make_list_6_dotted() -> List<TestVal> {
        let cell = List::new(TestVal::Int(5), Some(TestVal::Int(6)));
        let cell2 = List::new(TestVal::Int(4), Some(TestVal::Pair(cell.clone())));
        let cell3 = List::new(TestVal::Int(3), Some(TestVal::Pair(cell2.clone())));
        let cell4 = List::new(TestVal::Int(2), Some(TestVal::Pair(cell3.clone())));
        let cell5 = List::new(TestVal::Int(1), Some(TestVal::Pair(cell4.clone())));
        cell5
    }

    // List //

    #[test]
    fn test_cell_head_and_tail() {
        let cell = List::new(TestVal::Int(5), None);
        assert_eq!(cell.head().clone(), TestVal::Int(5));
        assert_eq!(cell.tail().clone(), None);
    }

    #[test]
    fn test_cell_is_dotted() {
        let cell = List::new(TestVal::Int(5), None);
        assert_eq!(cell.is_dotted(), false);

        let cell = List::new(
            TestVal::Int(5),
            Some(TestVal::Pair(List::new(TestVal::Int(9), None))),
        );
        assert_eq!(cell.is_dotted(), false);

        let cell = List::new(TestVal::Int(5), Some(TestVal::Int(6)));
        assert_eq!(cell.is_dotted(), true);
    }

    // Iterators //

    #[test]
    fn test_cell_iterator() {
        let cell = List::new(TestVal::Int(5), None);
        let cell2 = List::new(TestVal::Int(4), Some(TestVal::Pair(cell.clone())));
        let cell3 = List::new(TestVal::Int(3), Some(TestVal::Pair(cell2.clone())));
        let cell4 = List::new(TestVal::Int(2), Some(TestVal::Pair(cell3.clone())));
        let cell5 = List::new(TestVal::Int(1), Some(TestVal::Pair(cell4.clone())));

        let mut iter = cell5.cells();
        assert_eq!(iter.next(), Some(cell5));
        assert_eq!(iter.next(), Some(cell4));
        assert_eq!(iter.next(), Some(cell3));
        assert_eq!(iter.next(), Some(cell2));
        assert_eq!(iter.next(), Some(cell));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_cell_value_iterator() {
        let list = make_list_5();
        let mut iter = list.values();
        assert_eq!(iter.next(), Some(TestVal::Int(1)));
        assert_eq!(iter.next(), Some(TestVal::Int(2)));
        assert_eq!(iter.next(), Some(TestVal::Int(3)));
        assert_eq!(iter.next(), Some(TestVal::Int(4)));
        assert_eq!(iter.next(), Some(TestVal::Int(5)));
        assert_eq!(iter.next(), None);

        let list = make_list_6_dotted();
        let mut iter = list.values();
        assert_eq!(iter.next(), Some(TestVal::Int(1)));
        assert_eq!(iter.next(), Some(TestVal::Int(2)));
        assert_eq!(iter.next(), Some(TestVal::Int(3)));
        assert_eq!(iter.next(), Some(TestVal::Int(4)));
        assert_eq!(iter.next(), Some(TestVal::Int(5)));
        assert_eq!(iter.next(), Some(TestVal::Int(6)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_representations() {
        let list = make_list_5();
        let list2 = make_list_6_dotted();
        let list3 = TestVal::Pair(List::new(
            TestVal::Pair(list2.clone()),
            Some(TestVal::Pair(list.clone())),
        ));
        assert_eq!(list3.to_display(), "((1 2 3 4 5 . 6) 1 2 3 4 5)".to_owned());
        assert_eq!(
            list3.to_external(),
            "((#1 #2 #3 #4 #5 . #6) #1 #2 #3 #4 #5)".to_owned()
        );
    }
}
*/
