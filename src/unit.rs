use std::fmt;
use std::fmt::Display;

pub fn px<T: Display>(t: T) -> Unit<T> {
    Unit {
        u: UnitType::Px,
        v: t,
    }
}

pub struct Unit<T: Display> {
    u: UnitType,
    pub v: T,
}

impl<T: Display> Display for Unit<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.v, self.u)
    }
}

pub enum UnitType {
    Px,
    Mm,
}

impl Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnitType::*;
        let s = match self {
            Px => "px",
            Mm => "mm",
        };
        write!(f, "{}", s)
    }
}
