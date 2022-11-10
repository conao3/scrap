use std::{rc::{Rc, Weak}, cell::RefCell, fmt::Display};

type RispExpRef = Weak<RefCell<RispExp>>;
type RispExpRefStrong = Rc<RefCell<RispExp>>;

struct Arena(Vec<RispExpRefStrong>);

impl Arena {
    fn new() -> Self {
        Self(Vec::with_capacity(100))
    }

    fn alloc(&mut self, exp: RispExp) -> RispExpRef {
        let rc = Rc::new(RefCell::new(exp));
        self.0.push(rc.clone());
        Rc::downgrade(&rc)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RispAtom {
    Int(i64),
    Symbol(String),
}

impl Display for RispAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RispAtom::Int(i) => write!(f, "{}", i),
            RispAtom::Symbol(s) => write!(f, "{}", s),
        }
    }
}

impl From<i64> for RispAtom {
    fn from(i: i64) -> Self {
        RispAtom::Int(i)
    }
}

impl From<&str> for RispAtom {
    fn from(s: &str) -> Self {
        RispAtom::Symbol(s.to_string())
    }
}

#[derive(Debug)]
pub enum RispExp {
    Atom(RispAtom),
    Cons{
        car: Weak<RefCell<RispExp>>,
        cdr: Weak<RefCell<RispExp>>,
    },
}

impl Display for RispExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RispExp::Atom(a) => write!(f, "{}", a),
            RispExp::Cons{car, cdr} => {
                write!(f, "({} . {})", car.upgrade().unwrap().borrow(), cdr.upgrade().unwrap().borrow())
            }
        }
    }
}

impl<T> From<T> for RispExp where T: Into<RispAtom> {
    fn from(t: T) -> Self {
        RispExp::Atom(t.into())
    }
}

impl From<(&RispExpRef, &RispExpRef)> for RispExp {
    fn from((car, cdr): (&RispExpRef, &RispExpRef)) -> Self {
        RispExp::Cons{car: car.clone(), cdr: cdr.clone()}
    }
}


fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let mut arena = Arena::new();

    // cons
    let nil = arena.alloc("nil".into());
    let v1 = arena.alloc(5.into());
    let v2 = arena.alloc(6.into());
    let v3 = arena.alloc(10.into());

    let a = arena.alloc((&v1, &nil).into());
    let b = arena.alloc((&v2, &a).into());
    let c = arena.alloc((&v3, &a).into());

    assert_eq!(a.upgrade().unwrap().borrow().to_string(), "(5 . nil)");
    assert_eq!(b.upgrade().unwrap().borrow().to_string(), "(6 . (5 . nil))");
    assert_eq!(c.upgrade().unwrap().borrow().to_string(), "(10 . (5 . nil))");

    // modify atom
    // 新規のRispExp::Atomで上書き
    let v1_ptr = v1.upgrade().unwrap();
    *v1_ptr.borrow_mut() = 15.into();

    assert_eq!(a.upgrade().unwrap().borrow().to_string(), "(15 . nil)");
    assert_eq!(b.upgrade().unwrap().borrow().to_string(), "(6 . (15 . nil))");
    assert_eq!(c.upgrade().unwrap().borrow().to_string(), "(10 . (15 . nil))");

    // modify cons
    // 新規のRispExp::Consで上書き
    let w1 = arena.alloc(42.into());
    let w2 = arena.alloc(43.into());

    let a_ptr = a.upgrade().unwrap();
    *a_ptr.borrow_mut() = (&w1, &w2).into();

    assert_eq!(a.upgrade().unwrap().borrow().to_string(), "(42 . 43)");
    assert_eq!(b.upgrade().unwrap().borrow().to_string(), "(6 . (42 . 43))");
    assert_eq!(c.upgrade().unwrap().borrow().to_string(), "(10 . (42 . 43))");

    // modify car
    // 既にArenaにあるAtomを指すように変更
    let x1 = arena.alloc(9.into());

    let a_ptr = a.upgrade().unwrap();
    match *a_ptr.borrow_mut() {
        RispExp::Cons { ref mut car, .. } => *car = x1,
        _ => panic!("not cons"),
    }

    assert_eq!(a.upgrade().unwrap().borrow().to_string(), "(9 . 43)");
    assert_eq!(b.upgrade().unwrap().borrow().to_string(), "(6 . (9 . 43))");
    assert_eq!(c.upgrade().unwrap().borrow().to_string(), "(10 . (9 . 43))");

    Ok(())
}
