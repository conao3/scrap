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
        car: RispExpRef,
        cdr: RispExpRef,
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

impl RispExp {
    pub fn car(&self) -> anyhow::Result<RispExpRefStrong> {
        match self {
            RispExp::Atom(_) => anyhow::bail!("car of atom"),
            RispExp::Cons{car, ..} => Ok(car.upgrade().unwrap()),
        }
    }

    pub fn car_weak(&self) -> anyhow::Result<RispExpRef> {
        match self {
            RispExp::Atom(_) => anyhow::bail!("car of atom"),
            RispExp::Cons{car, ..} => Ok(car.clone()),
        }
    }

    pub fn car_weak_ref(&self) -> anyhow::Result<&RispExpRef> {
        match self {
            RispExp::Atom(_) => anyhow::bail!("car of atom"),
            RispExp::Cons{car, ..} => Ok(car),
        }
    }

    pub fn cdr(&self) -> anyhow::Result<RispExpRefStrong> {
        match self {
            RispExp::Atom(_) => anyhow::bail!("cdr of atom"),
            RispExp::Cons{cdr, ..} => Ok(cdr.upgrade().unwrap()),
        }
    }

    pub fn cdr_weak(&self) -> anyhow::Result<RispExpRef> {
        match self {
            RispExp::Atom(_) => anyhow::bail!("cdr of atom"),
            RispExp::Cons{cdr, ..} => Ok(cdr.clone()),
        }
    }

    pub fn cdr_weak_ref(&self) -> anyhow::Result<&RispExpRef> {
        match self {
            RispExp::Atom(_) => anyhow::bail!("cdr of atom"),
            RispExp::Cons{cdr, ..} => Ok(cdr),
        }
    }

    pub fn into_iter(&self) -> RispExpIter {
        RispExpIter{car: self.car_weak().ok(), cdr: self.cdr_weak().ok()}
    }
}

pub struct RispExpIter {
    car: Option<RispExpRef>,
    cdr: Option<RispExpRef>,
}

impl Iterator for RispExpIter {
    type Item = RispExpRefStrong;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(car_val) = self.car.take() {
            if let Some(cdr_val) = self.cdr.take() {
                let cdr_ptr = cdr_val.upgrade().unwrap();
                let cdr = cdr_ptr.borrow();
                //let cdr = cdr.borrow();
                self.car = cdr.car_weak_ref().ok().cloned();
                self.cdr = cdr.cdr_weak_ref().ok().cloned();
            }
            Some(car_val.upgrade().unwrap())
        } else {
            None
        }
    }
}


fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let mut arena = Arena::new();

    let nil = arena.alloc(RispExp::Atom("nil".into()));
    let c1 = arena.alloc(1.into());
    let c2 = arena.alloc(2.into());
    let c3 = arena.alloc(3.into());

    let e1 = arena.alloc((&c1, &c2).into());
    let e2 = arena.alloc((&e1, &c3).into());

    println!("e2: {}", e2.upgrade().unwrap().borrow());

    let a1 = e2.upgrade().unwrap().borrow().car()?;
    println!("a1: {}", a1.borrow());

    let a2 = a1.borrow().car()?;
    println!("a2: {}", a2.borrow());


    // method chain pattern
    let a3 = e2.upgrade().unwrap().borrow().car()?.borrow().car()?;
    println!("a3: {}", a3.borrow());


    // iter
    let f1 = arena.alloc((&c1, &nil).into());
    let f2 = arena.alloc((&c2, &f1).into());
    let f3 = arena.alloc((&c3, &f2).into());

    for e in f3.upgrade().unwrap().borrow().into_iter() {
        println!("iter: {}", e.borrow());
    }

    Ok(())
}
