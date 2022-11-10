use std::{rc::Rc, rc::Weak, cell::RefCell, fmt::Display};

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
            RispExp::Cons{car: car_, cdr: cdr_} => {
                let car = car_.upgrade().unwrap();
                let cdr = cdr_.upgrade().unwrap();
                let msg = format!("({} . {})", car.borrow(), cdr.borrow());
                write!(f, "{}", msg)
            }
        }
    }
}

impl<T> From<T> for RispExp where T: Into<RispAtom> {
    fn from(t: T) -> Self {
        RispExp::Atom(t.into())
    }
}

impl RispExp {
    pub fn atom<T>(a: T) -> Self
    where
        T: Into<RispAtom>,
    {
        RispExp::Atom(a.into())
    }

    pub fn cons<S, T>(car: S, cdr: T) -> Self
    where
        S: Into<RispExp>,
        T: Into<RispExp>,
    {
        let car = Rc::new(RefCell::new(car.into()));
        let cdr = Rc::new(RefCell::new(cdr.into()));
        RispExp::Cons{
            car: Rc::downgrade(&car),
            cdr: Rc::downgrade(&cdr),
        }
    }

    pub fn car(&self) -> anyhow::Result<Rc<RefCell<RispExp>>> {
        match self {
            RispExp::Atom(_) => anyhow::bail!("car of atom"),
            RispExp::Cons{car, ..} => Ok(car.upgrade().unwrap()),
        }
    }
}


fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let e1 = RispExp::atom(1);
    let e2 = RispExp::cons(e1, 2);
    let e3 = RispExp::cons(e2, 3);

    println!("e3: {}", e3);

    let c1 = e3.car()?;
    println!("c1: {}", c1.borrow());

    //let c2 = e3.car()?.car()?;
    let c2 = e3.car()?.borrow().car()?;
    println!("c2: {}", c2.borrow());

    Ok(())
}
