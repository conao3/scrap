use std::{rc::{Weak, Rc}, cell::RefCell, fmt::Display};

type RispExpRef = Option<Weak<RefCell<Cell>>>;
type RispExpRefStrong = Option<Rc<RefCell<Cell>>>;

struct Arena(Vec<RispExpRefStrong>);

impl Arena {
    fn new() {
        Arena(Vec::with_capacity(1000));
    }

    fn alloc(&mut self, cell: Cell) -> RispExpRef {
        let rc = Rc::new(RefCell::new(cell));
        self.0.push(Some(rc.clone()));
        Some(Rc::downgrade(&rc))
    }
}

struct RispExp(RispExpRef);

struct Cell {
    car: RispExpRef,
    cdr: RispExpRef,
}

enum RispAtom {
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

impl From<i64> for Cell {
    
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell { car: Some(car), cdr: Some(cdr) } => {
                let car_ptr = car.upgrade().unwrap();
                let cdr_ptr = cdr.upgrade().unwrap();
                let car_ref = car_ptr.borrow();
                let cdr_ref = cdr_ptr.borrow();
                write!(f, "({} . {})", car_ref, cdr_ref)
            },
            Cell { car: Some(car), cdr: None } => {
                let car_ptr = car.upgrade().unwrap();
                let car_ref = car_ptr.borrow();
                write!(f, "{}", car_ref)
            },
            Cell { car: None, cdr: Some(..) } => {
                unreachable!("cdr without car");
            },
            Cell { car: None, cdr: None } => write!(f, "nil"),
        }
    }
}

impl Display for RispExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(exp) => {
                let exp_ptr = exp.upgrade().unwrap();
                let exp_ref = exp_ptr.borrow();
                write!(f, "{}", exp_ref)
            },
            None => write!(f, "nil"),
        }
    }
}

impl RispExp {
    fn new() -> Self {
        RispExp(None)
    }

    fn cons(&mut self, car: RispExpRef) {

    }
}

fn main() {
    println!("Hello, world!");
}
