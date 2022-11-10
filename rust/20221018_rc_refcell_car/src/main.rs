use std::{rc::Rc, cell::RefCell, cell::Ref, fmt::Display};

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

#[derive(Debug, PartialEq, Eq)]
pub enum RispExp {
    Atom(RispAtom),
    Cons{
        car: Rc<RefCell<RispExp>>,
        cdr: Rc<RefCell<RispExp>>,
    },
}

impl Display for RispExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RispExp::Atom(a) => write!(f, "{}", a),
            RispExp::Cons{car, cdr} => {
                write!(f, "({} . {})", car.borrow(), cdr.borrow())
            }
        }
    }
}

impl<T> From<T> for RispExp where T: Into<RispAtom> {
    fn from(t: T) -> Self {
        RispExp::Atom(t.into())
    }
}

type RispExpRef = Rc<RefCell<RispExp>>;

impl RispExp {
    pub fn atom<T>(a: T) -> RispExpRef
    where
        T: Into<RispAtom>,
    {
        Rc::new(RefCell::new(RispExp::Atom(a.into())))
    }

    pub fn cons(car: &RispExpRef, cdr: &RispExpRef) -> RispExpRef
    {
        Rc::new(RefCell::new(RispExp::Cons{
            car: car.clone(),
            cdr: cdr.clone(),
        }))
    }

    pub fn car(&self) -> anyhow::Result<RispExpRef> {
        match self {
            RispExp::Atom(_) => anyhow::bail!("car of atom"),
            RispExp::Cons{car, ..} => Ok(car.clone()),
        }
    }

    pub fn car_ref(&self) -> anyhow::Result<&RispExpRef> {
        match self {
            RispExp::Atom(_) => anyhow::bail!("car of atom"),
            RispExp::Cons{car, ..} => Ok(car),
        }
    }

    pub fn cdr(&self) -> anyhow::Result<RispExpRef> {
        match self {
            RispExp::Atom(_) => anyhow::bail!("cdr of atom"),
            RispExp::Cons{cdr, ..} => Ok(cdr.clone()),
        }
    }

    pub fn cdr_ref(&self) -> anyhow::Result<&RispExpRef> {
        match self {
            RispExp::Atom(_) => anyhow::bail!("cdr of atom"),
            RispExp::Cons{cdr, ..} => Ok(cdr),
        }
    }
}


fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let c1 = RispExp::atom(1);
    let c2 = RispExp::atom(2);
    let c3 = RispExp::atom(3);

    let e1 = RispExp::cons(&c1, &c2);
    let e2 = RispExp::cons(&e1, &c3);

    println!("e2: {}", e2.borrow());

    let a1 = e2.borrow().car()?;
    println!("a1: {}", a1.borrow());

    let a2 = a1.borrow().car()?;
    println!("a2: {}", a2.borrow());


    // method chain pattern
    let a3 = e2.borrow().car()?.borrow().car()?;
    println!("a3: {}", a3.borrow());

    // use car_ref
    let f1 = RispExp::cons(&c1, &c2);
    let f2 = RispExp::cons(&f1, &c3);

    let a4_bind1 = f2.borrow();
    let a4_bind2 = a4_bind1.car_ref()?.borrow();
    let a4 = a4_bind2.car_ref()?;
    println!("a4: {}", a4.borrow());

    Ok(())
}
