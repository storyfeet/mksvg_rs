//! The aim of the args method is to make a list of svg arguments
//! easy to compose
//!
//! They also remove the complication of separating styles,
//! transforms, and standard xml arguments
//!
//! each args function consumes, modifies and returns it's input
//! so that they can be chained.
//!
//! ```
//! use mksvg::args::{Args,SvgArg};
//! let a = Args::new().x(4).stroke("black").translate(4,5);
//! assert_eq!(r#"x="4" style="stroke:black;" transform="translate(4,5) " "#,
//! &format!("{}",a));
//! ```
//!
//! the SvgWrite methods (from mod write) accept an Args object.

use std::fmt;
use std::fmt::{Display, Formatter};
use std::marker::Sized;

#[derive(Copy, Clone, Debug, PartialEq)]
enum AType {
    ARG,
    STYLE,
    TRANS,
}

use self::AType::*;

#[derive(Clone, Debug, PartialEq)]
struct Arg {
    k: String,
    v: String,
    tp: AType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Args {
    items: Vec<Arg>,
}

/// # example
///
/// ```
/// use mksvg::args::{Args,SvgArg};
/// let a = Args::new().arg("f",7).style("p","rt");
/// assert_eq!(r#"f="7" style="p:rt;" "#,&format!("{}",a));
/// ```
impl Args {
    pub fn new() -> Args {
        Args { items: Vec::new() }
    }
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut astr = "".to_string();
        let mut sstr = "".to_string();
        let mut tstr = "".to_string();
        for a in &self.items {
            match a.tp {
                ARG => astr.push_str(&format!(r#"{}="{}" "#, &a.k, a.v)),
                STYLE => sstr.push_str(&format!("{}:{};", &a.k, a.v)),
                TRANS => tstr.push_str(&format!("{}({}) ", &a.k, a.v)),
            }
        }
        if sstr.len() > 0 {
            astr.push_str(&format!(r#"style="{}" "#, &sstr));
        }
        if tstr.len() > 0 {
            astr.push_str(&format!(r#"transform="{}" "#, &tstr));
        }
        write!(f, "{}", astr)
    }
}

impl SvgArg for Args {
    fn arg<T: Display>(mut self, k: &str, v: T) -> Self {
        self.items.push(Arg {
            k: k.to_string(),
            v: v.to_string(),
            tp: ARG,
        });
        self
    }
    fn style<T: Display>(mut self, k: &str, v: T) -> Self {
        self.items.push(Arg {
            k: k.to_string(),
            v: format!("{}", v),
            tp: STYLE,
        });
        self
    }
    fn transform<T: Display>(mut self, k: &str, args: &[T]) -> Self {
        let mut vstr = "".to_string();
        let mut first = true;
        for s in args {
            if first {
                vstr.push_str(&format!("{}", s));
                first = false;
                continue;
            }
            vstr.push_str(&format!(",{}", s));
        }
        self.items.push(Arg {
            k: k.to_string(),
            v: vstr,
            tp: TRANS,
        });
        self
    }
}

///
///```
///use mksvg::args::{Args,SvgArg};
///let a = Args::new().stroke_width(2);
///assert_eq!(r#"style="stroke-width:2;" "#,&format!("{}",a));
///```
pub trait SvgArg: Sized + Display {
    fn arg<T: Display>(self, k: &str, v: T) -> Self;
    fn style<T: Display>(self, k: &str, v: T) -> Self;
    fn transform<T: Display>(self, k: &str, args: &[T]) -> Self;

    //styles

    fn font_size<T: Display>(self, n: T) -> Self {
        self.style("font-size", n)
    }
    fn font_family<T: Display>(self, n: T) -> Self {
        self.style("font-family", n)
    }
    fn stroke_width<T: Display>(self, n: T) -> Self {
        self.style("stroke-width", n)
    }
    fn stroke<T: Display>(self, n: T) -> Self {
        self.style("stroke", n)
    }
    fn fill<T: Display>(self, n: T) -> Self {
        self.style("fill", n)
    }
    fn font_weight<T: Display>(self, n: T) -> Self {
        self.style("font-weight", n)
    }

    //args

    fn d<T: Display>(self, n: T) -> Self {
        self.arg("d", n)
    }
    fn id<T: Display>(self, n: T) -> Self {
        self.arg("id", n)
    }
    fn x<T: Display>(self, n: T) -> Self {
        self.arg("x", n)
    }
    fn y<T: Display>(self, n: T) -> Self {
        self.arg("y", n)
    }
    fn xy<T:Display>(self,x:T,y:T)->Self{
        self.x(x).y(y)
    }
    fn cy<T: Display>(self, n: T) -> Self {
        self.arg("cy", n)
    }
    fn cx<T: Display>(self, n: T) -> Self {
        self.arg("cx", n)
    }
    fn rx<T: Display>(self, n: T) -> Self {
        self.arg("rx", n)
    }
    fn ry<T: Display>(self, n: T) -> Self {
        self.arg("ry", n)
    }
    fn width<T: Display>(self, n: T) -> Self {
        self.arg("width", n)
    }
    fn height<T: Display>(self, n: T) -> Self {
        self.arg("height", n)
    }
    fn wh<T:Display>(self,w:T,h:T) ->Self{
        self.w(w).h(h)
    }
    fn href<T: Display>(self, n: T) -> Self {
        self.arg("xlink:href", n)
    }
    fn text_anchor<T: Display>(self, n: T) -> Self {
        self.arg("text-anchor", n)
    }

    //transforms

    fn rotate<T: Display>(self, ang: T, x: T, y: T) -> Self {
        self.transform("rotate", &[ang, x, y])
    }
    fn translate<T: Display>(self, x: T, y: T) -> Self {
        self.transform("translate", &[x, y])
    }
    fn scale<T: Display>(self, x: T, y: T) -> Self {
        self.transform("scale", &[x, y])
    }

    fn skew_x<T: Display>(self, x: T) -> Self {
        self.transform("skewX", &[x])
    }
    fn scew_y<T: Display>(self, y: T) -> Self {
        self.transform("skewY", &[y])
    }
    fn matrix<T: Display>(self, args: &[T]) -> Self {
        self.transform("scale", args)
    }
    //shorners

    fn w<T: Display>(self, n: T) -> Self {
        self.width(n)
    }
    fn h<T: Display>(self, n: T) -> Self {
        self.height(n)
    }
    fn t_anc<T: Display>(self, n: T) -> Self {
        self.text_anchor(n)
    }
}
