use std::{rc::Rc, cell::RefCell, fmt::Display};

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

type RispExpRef = Rc<RefCell<RispExp>>;

#[derive(Debug, PartialEq, Eq)]
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

    pub fn iter(&self) -> RispExpIter {
        match self {
            RispExp::Atom(_) => panic!(),
            RispExp::Cons{car, cdr} => RispExpIter {
                car: Some(car),
                cdr: Some(cdr),
                tmp: None,
            }
        }
    }

    // pub fn into_iter(&self) -> RispExpIntoIter {
    //     match self {
    //         RispExp::Atom(_) => panic!(),
    //         RispExp::Cons{car, cdr} => RispExpIntoIter {
    //             car: Some(car.clone()),
    //             cdr: Some(cdr.clone()),
    //         }
    //     }
    // }
}

// pub struct RispExpIntoIter {car: Option<RispExpRef>, cdr: Option<RispExpRef>}

// impl Iterator for RispExpIntoIter {
//     type Item = RispExpRef;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(car_val) = self.car.take() {
//             if let Some(cdr_val) = self.cdr.take() {
//                 match &*cdr_val.borrow() {
//                     RispExp::Atom(..) => (),
//                     RispExp::Cons{car, cdr} => {
//                         self.car = Some(car.clone());
//                         self.cdr = Some(cdr.clone());
//                     }
//                 };
//             }
//             Some(car_val)
//         } else {
//             None
//         }
//     }
// }

pub struct RispExpIter<'a> {car: Option<&'a RispExpRef>, cdr: Option<&'a RispExpRef>, tmp: Option<RispExpRef>}

impl<'a> Iterator for RispExpIter<'a> {
    type Item = &'a RispExpRef;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(car_val) = self.car.take() {
            if let Some(cdr_val) = self.cdr.take() {
                self.tmp = Some(cdr_val.clone());
                match &*self.tmp.as_ref().unwrap().borrow() {
                    RispExp::Atom(..) => (),
                    RispExp::Cons{car, cdr} => {
                        self.car = Some(car);
                        self.cdr = Some(cdr);
                    }
                };
            }
            Some(car_val)
        } else {
            None
        }
    }
}

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let mut target = RispExp::atom("nil");
    for i in 0..3 {
        let c = RispExp::atom(i + 1);
        target = RispExp::cons(&c, &target);
    }

    println!("target: {}", target.borrow());

    for e in target.borrow().into_iter() {
        println!("e: {}", e.borrow());
    }

    Ok(())
}
