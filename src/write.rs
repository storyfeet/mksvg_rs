use crate::args::{Args, SvgArg};

use num;
use std::fmt::Display;

use std::fmt::Write as FmtWrite;
use std::io::Write as IOWrite;

pub trait CDNum: num::Num + num::NumCast + Copy + Display {}

impl<T: num::Num + num::NumCast + Copy + Display> CDNum for T {}

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

    ///writes a simple prelude for an svg file
    ///don't forget to call end.
    fn start<T: CDNum>(&mut self, w: T, h: T) {
        self.write("<?xml version=\"1.0\" ?>");
        self.any_o(
            "svg",
            &Args::new()
                .w(w)
                .h(h)
                .arg("xmlns", "http://www.w3.org/2000/svg")
                .arg("xmlns:xlink", "http://www.w3.org/1999/xlink"),
        );
    }

    ///ends the svg file
    fn end(&mut self) {
        self.inc_depth(-1);
        self.write("</svg>");
    }

    fn g(&mut self, args: Args) {
        self.any_o("g", &args);
    }

    fn g_translate<T: CDNum>(&mut self, x: T, y: T) {
        self.g(Args::new().translate(x, y));
    }

    fn g_rotate<T: CDNum>(&mut self, ang: T, x: T, y: T) {
        self.g(Args::new().rotate(ang, x, y));
    }

    fn g_end(&mut self) {
        self.inc_depth(-1);
        self.write("</g>");
    }

    fn any(&mut self, name: &str, args: &Args) {
        self.write(&format!("<{} {}/>", name, args));
    }

    fn any_o(&mut self, name: &str, args: &Args) {
        self.write(&format!("<{} {}>", name, args));
        self.inc_depth(1);
    }

    fn rect<T: CDNum>(&mut self, x: T, y: T, w: T, h: T, args: Args) {
        self.any("rect", &args.x(x).y(y).w(w).h(h));
    }

    fn ellipse<T: CDNum>(&mut self, cx: T, cy: T, rx: T, ry: T, args: Args) {
        self.any("ellipse", &args.cx(cx).cy(cy).rx(rx).ry(ry));
    }

    fn text<T: CDNum>(&mut self, tx: &str, x: T, y: T, fs: T, args: Args) {
        self.write(&format!(
            "<text {} >{}</text>",
            args.x(x).y(y).font_size(fs),
            tx
        ));
    }

    /// text_lines provides a simple way of printing multiline text
    /// dy is the distance down y has to be.
    fn text_lines<T: CDNum>(&mut self, tx: &str, x: T, y: T, fs: T, dy: T, args: Args) {
        let lns = tx.split("\n");
        let mut ln_y: T = y;
        for ln in lns {
            self.text(&ln, x, ln_y, fs, args.clone().y(ln_y));
            ln_y = ln_y + dy;
        }
    }

    /// svg has a foible, where a wide stroke on text, hides the text.
    /// this method provides a workaround, printing the text twice.
    /// the back copy has the stroke, so all the front letters remain intact.
    fn bg_text<T: CDNum>(&mut self, tx: &str, x: T, y: T, fs: T, sw: T, scol: &str, args: Args) {
        self.text(tx, x, y, fs, args.clone().stroke_width(sw).stroke(scol));
        self.text(tx, x, y, fs, args.stroke("none"));
    }

    /// multiple lines with the background workaround
    fn bg_text_lines<T: CDNum>(
        &mut self,
        tx: &str,
        x: T,
        y: T,
        fs: T,
        dy: T,
        sw: T,
        scol: &str,
        args: Args,
    ) {
        let p = args.font_size(fs);
        self.text_lines(tx, x, y, fs, dy, p.clone().stroke_width(sw).stroke(scol));
        self.text_lines(tx, x, y, fs, dy, p.stroke("none"));
    }

    fn img<T: Display>(&mut self, loc: &str, x: T, y: T, w: T, h: T) {
        self.any("image", &Args::new().x(x).y(y).w(w).h(h).href(loc));
    }

    fn path<T: Display>(&mut self, pathd: T, args: Args) {
        self.any("path", &args.d(pathd));
    }
}


pub struct TransWrap<'a,W:'a +SvgWrite>{
    td_inc:i8,
    end:&'static str,
    w:&'a mut W,
}

impl<'a,W:SvgWrite> TransWrap<'a,W>{
    pub fn new(w:&'a mut W,begin:&str,end:&'static str)->Self{
        w.write(begin);
        w.inc_depth(1);

        TransWrap{
            td_inc:1,
            end,
            w,
        }
    }
}

impl<'a,W:SvgWrite> SvgWrite for TransWrap<'a,W>{
    fn write(&mut self, s: &str){
        self.w.write(s);
    }
    fn inc_depth(&mut self, n: i8){
        self.td_inc += n;
        self.w.inc_depth(n);
    }
}

impl<'a,W:SvgWrite> Drop for TransWrap<'a,W>{
    fn drop(&mut self){
        self.w.inc_depth(-self.td_inc);
        self.w.write(self.end);
    }
}



#[cfg(test)]
mod test{
    use super::*;
    #[test]
    pub fn test_wrapper(){
        let mut buf = String::new();
        let mut g = SvgFmt::new(&mut buf);
        let mut p = TransWrap::new(&mut g,"<hello>","<goodbye>");
        p.write("<item>");
        
        drop(p);
        assert_eq!(&buf,"<hello>\n  <item>\n<goodbye>\n");
        
    }
}
