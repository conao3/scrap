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

impl From<(RispExpRef, RispExpRef)> for RispExp {
    fn from((car, cdr): (RispExpRef, RispExpRef)) -> Self {
        RispExp::Cons{car, cdr}
    }
}

macro_rules! alloc {
    ($arena: ident, [$exp: tt]) => {{
        let e = alloc!($arena, $exp);
        let nil = $arena.alloc("nil".into());
        $arena.alloc((e, nil).into())
    }};
    ($arena: ident, [$car: tt, $cdr: tt]) => {{
        let car = alloc!($arena, $car);
        let cdr = alloc!($arena, $cdr);
        $arena.alloc((car, cdr).into())
    }};
    ($arena: ident, [$car: tt, $($rest: tt),*]) => {{
        let car = alloc!($arena, $car);
        let cdr = alloc!($arena, [$($rest),*]);
        $arena.alloc((car, cdr).into())
    }};
    ($arena: ident, $exp: tt) => {
        $exp.clone()
    };
}

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let mut arena = Arena::new();

    // cons
    let nil = arena.alloc("nil".into());
    let v1 = arena.alloc(5.into());

    let a = arena.alloc((&v1, &nil).into());

    assert_eq!(a.upgrade().unwrap().borrow().to_string(), "(5 . nil)");

    ////

    let v1 = arena.alloc(1.into());
    let v2 = arena.alloc(2.into());
    let v3 = arena.alloc(3.into());

    let exp_a = alloc!(arena, [v1]);
    println!("exp_a: {}", exp_a.upgrade().unwrap().borrow());

    let exp_a = alloc!(arena, [v1, v2]);
    println!("exp_a: {}", exp_a.upgrade().unwrap().borrow());

    let exp_a = alloc!(arena, [[v1, v2], v3]);
    println!("exp_a: {}", exp_a.upgrade().unwrap().borrow());

    let exp_a = alloc!(arena, [v1, [v2, v3]]);
    println!("exp_a: {}", exp_a.upgrade().unwrap().borrow());

    let exp_quote = arena.alloc("quote".into());
    let exp_a = arena.alloc("a".into());
    let exp_ldc = arena.alloc("ldc".into());
    let exp_code = alloc!(arena, [exp_quote, exp_a]);
    let exp_a = alloc!(arena, [exp_ldc, exp_code]);
    println!("exp_a: {}", exp_a.upgrade().unwrap().borrow());

    let exp_a = alloc!(arena, [exp_ldc, [exp_code]]);
    println!("exp_a: {}", exp_a.upgrade().unwrap().borrow());

    Ok(())
}
