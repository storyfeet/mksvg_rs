use num;
use std::fmt::{Debug, Display};

use std::fmt::Write as FmtWrite;
use std::io::Write as IOWrite;

pub trait CDNum: num::Num + num::NumCast + Copy + Display + Debug {}

impl<T: num::Num + num::NumCast + Copy + Display + Debug> CDNum for T {}

///qcast (quick cast) is makes it simple to use primitives with an unknown CDNum type
///
/// ```
/// use mksvg::write::{qcast,CDNum};
/// fn f<T:CDNum>(n:T)->T{ n + qcast(20)}
/// assert_eq!(f(1.0),21.0);
/// assert_eq!(f(1),21);
/// ```
///
pub fn qcast<A: CDNum, B: CDNum>(a: A) -> B {
    num::NumCast::from(a).unwrap()
}

/// SvgIO is a very simple writer, that takes an std::io::Write keeps a tab depth.
/// it implements SvgWrite,  and prints the lines given at a the current depth
pub struct SvgIO<W: IOWrite> {
    w: W,
    d: i8,
}

impl<W: IOWrite> SvgIO<W> {
    pub fn new(w: W) -> SvgIO<W> {
        SvgIO { w: w, d: 0 }
    }
    fn pad(&self) -> String {
        let mut res = "".to_string();
        for _i in 0..self.d {
            res.push_str("  ");
        }
        res
    }
}

impl<W: IOWrite> SvgWrite for SvgIO<W> {
    fn write(&mut self, s: &str) {
        let ps = self.pad();
        write!(self.w, "{}{}\n", ps, s).unwrap();
    }
    fn inc_depth(&mut self, n: i8) {
        self.d += n;
    }
}

/// SvgFmt is a very simple writer, that takes an std::fmt::Write aand keeps a tab depth.
/// it implements SvgWrite, and prints the lines given at a the current depth
pub struct SvgFmt<W: FmtWrite> {
    w: W,
    d: i8,
}

impl<W: FmtWrite> SvgFmt<W> {
    pub fn new(w: W) -> SvgFmt<W> {
        SvgFmt { w: w, d: 0 }
    }
    fn pad(&self) -> String {
        let mut res = "".to_string();
        for _i in 0..self.d {
            res.push_str("  ");
        }
        res
    }
}

impl<W: FmtWrite> SvgWrite for SvgFmt<W> {
    fn write(&mut self, s: &str) {
        let ps = self.pad();
        write!(self.w, "{}{}\n", ps, s).unwrap();
    }
    fn inc_depth(&mut self, n: i8) {
        self.d += n;
    }
}
/// the methods on SvgWrite, do not build any structure
/// they simply write the output, so if you open something (g or svg) don't forget to close it.
pub trait SvgWrite {
    fn write(&mut self, s: &str);
    fn inc_depth(&mut self, n: i8);
}

pub struct TransWrap<'a> {
    start: Option<String>,
    td_inc: i8,
    end: String,
    w: &'a mut SvgWrite,
}

impl<'a> TransWrap<'a> {
    pub fn new(w: &'a mut SvgWrite, begin: &str, end: &str) -> Self {
        TransWrap {
            start: Some(begin.to_string()),
            td_inc: 1,
            end: end.to_string(),
            w,
        }
    }

    pub fn force(&mut self) {
        if let Some(ref st) = self.start {
            self.w.write(&st);
            self.w.inc_depth(1);
            self.start = None;
        }
    }
}

impl<'a> SvgWrite for TransWrap<'a> {
    fn write(&mut self, s: &str) {
        self.force();
        self.w.write(s);
    }
    fn inc_depth(&mut self, n: i8) {
        self.td_inc += n;
        self.w.inc_depth(n);
    }
}

impl<'a> Drop for TransWrap<'a> {
    fn drop(&mut self) {
        if self.start == None {
            self.w.inc_depth(-self.td_inc);
            self.w.write(&self.end);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_wrapper() {
        let mut buf = String::new();
        let mut g = SvgFmt::new(&mut buf);
        let mut p = TransWrap::new(&mut g, "<hello>", "<goodbye>");
        p.write("<item>");

        drop(p);
        assert_eq!(&buf, "<hello>\n  <item>\n<goodbye>\n");
    }

    #[test]
    pub fn test_no_write() {
        let mut buf = String::new();
        let mut g = SvgFmt::new(&mut buf);
        let p = TransWrap::new(&mut g, "<hello>", "<goodbye>");

        drop(p);
        assert_eq!(&buf, "");
    }

    #[test]
    pub fn test_force() {
        let mut buf = String::new();
        let mut g = SvgFmt::new(&mut buf);
        let mut p = TransWrap::new(&mut g, "<hello>", "<goodbye>");
        p.force();
        drop(p);
        assert_eq!(&buf, "<hello>\n<goodbye>\n");
    }
}
