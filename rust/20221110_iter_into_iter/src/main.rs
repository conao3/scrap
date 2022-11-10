use std::fmt::Display;

type Link = Option<Box<Node>>;

struct List(Link);

struct Node {
    val: String,
    next: Link,
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cur = &self.0;
        while let Some(node) = cur {
            write!(f, "{} -> ", node.val)?;
            cur = &node.next;
        }
        write!(f, "None")
    }
}

impl List {
    fn new() -> Self {
        List(None)
    }

    fn push(&mut self, val: String) {
        let new_node = Box::new(Node {
            val,
            next: self.0.take(),
        });
        self.0 = Some(new_node);
    }

    fn pop(&mut self) -> Option<String> {
        self.0.take().map(|node| {
            self.0 = node.next;
            node.val
        })
    }

    fn peek(&self) -> Option<&String> {
        self.0.as_ref().map(|node| &node.val)
    }
}

// IntoIter
struct IntoIter(List);

impl Iterator for IntoIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = &mut self.0;
        cur.0.take().map(|node| {
            self.0.0 = node.next;
            node.val
        })
    }
}

impl IntoIterator for List {
    type Item = String;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl List {
    fn into_iter(self) -> IntoIter {
        IntoIter(self)
    }
}

// Iter
struct Iter<'a>(&'a Link);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_ref().map(|node| {
            self.0 = &node.next;
            &node.val
        })
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = &'a String;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl List {
    fn iter(&self) -> Iter {
        Iter(&self.0)
    }
}

// IterMut
struct IterMut<'a>(Option<&'a mut Node>);

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref_mut();
            &mut node.val
        })
    }
}

impl<'a> IntoIterator for &'a mut List {
    type Item = &'a mut String;
    type IntoIter = IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl List {
    fn iter_mut(&mut self) -> IterMut {
        IterMut(self.0.as_deref_mut())
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        let mut lst = List::new();
        assert_eq!(lst.pop(), None);

        lst.push("a".to_string());
        lst.push("b".to_string());
        lst.push("c".to_string());

        assert_eq!(lst.pop(), Some("c".to_string()));
        assert_eq!(lst.pop(), Some("b".to_string()));

        lst.push("d".to_string());
        lst.push("e".to_string());

        assert_eq!(lst.pop(), Some("e".to_string()));
        assert_eq!(lst.pop(), Some("d".to_string()));
        assert_eq!(lst.pop(), Some("a".to_string()));
        assert_eq!(lst.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut lst = List::new();
        assert_eq!(lst.peek(), None);

        lst.push("a".to_string());
        lst.push("b".to_string());
        assert_eq!(lst.peek(), Some(&"b".to_string()));

        assert_eq!(lst.pop(), Some("b".to_string()));
    }

    #[test]
    fn test_into_iter() {
        fn new() -> List {
            let mut lst = List::new();
            lst.push("a".to_string());
            lst.push("b".to_string());
            lst.push("c".to_string());
            lst
        }

        // direct iterator usage
        let lst = new();
        let mut iter = lst.into_iter();
        assert_eq!(iter.next(), Some("c".to_string()));
        assert_eq!(iter.next(), Some("b".to_string()));
        assert_eq!(iter.next(), Some("a".to_string()));
        assert_eq!(iter.next(), None);

        // for-in usage
        let lst = new();
        let mut expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        for val in lst.into_iter() {
            assert_eq!(val, expected.pop().unwrap());
        }

        // for-in usage (implicit)
        let lst = new();
        let mut expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        for val in lst {
            assert_eq!(val, expected.pop().unwrap());
        }
    }

    #[test]
    fn test_iter() {
        fn new() -> List {
            let mut lst = List::new();
            lst.push("a".to_string());
            lst.push("b".to_string());
            lst.push("c".to_string());
            lst
        }

        // direct iterator usage
        let lst = new();
        let mut iter = lst.iter();
        assert_eq!(iter.next(), Some(&"c".to_string()));
        assert_eq!(iter.next(), Some(&"b".to_string()));
        assert_eq!(iter.next(), Some(&"a".to_string()));
        assert_eq!(iter.next(), None);

        // for-in usage
        let lst = new();
        let mut expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        for val in lst.iter() {
            assert_eq!(*val, expected.pop().unwrap());
        }

        // for-in usage (implicit)
        let lst = new();
        let mut expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        for val in &lst {
            assert_eq!(*val, expected.pop().unwrap());
        }
    }

    #[test]
    fn test_iter_mut() {
        fn new() -> List {
            let mut lst = List::new();
            lst.push("a".to_string());
            lst.push("b".to_string());
            lst.push("c".to_string());
            lst
        }

        // direct iterator usage
        let mut lst = new();
        let mut iter = lst.iter_mut();
        assert_eq!(iter.next(), Some(&mut "c".to_string()));
        assert_eq!(iter.next(), Some(&mut "b".to_string()));
        assert_eq!(iter.next(), Some(&mut "a".to_string()));
        assert_eq!(iter.next(), None);

        // for-in usage
        let mut lst = new();
        let mut expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        for val in lst.iter_mut() {
            assert_eq!(*val, expected.pop().unwrap());
        }

        // for-in usage (implicit)
        let mut lst = new();
        assert_eq!(lst.to_string(), "c -> b -> a -> None");

        let mut expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        for val in &mut lst {
            assert_eq!(*val, expected.pop().unwrap());
            if val == "b" {
                *val = "x".to_string();
            }
        }

        assert_eq!(lst.to_string(), "c -> x -> a -> None");
    }
}
