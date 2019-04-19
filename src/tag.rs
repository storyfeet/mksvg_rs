use crate::args::{Args, SvgArg};
use crate::write::SvgWrite;
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    name: &'static str,
    args: Args,
}

impl Tag {
    pub fn new(name: &'static str) -> Self {
        Tag {
            name,
            args: Args::new(),
        }
    }
    pub fn rect<P: Display, S: Display>(x: P, y: P, w: S, h: S) -> Self {
        Tag::new("rect").xy(x, y).wh(w, h)
    }
    pub fn img<P: Display, S: Display>(loc: &str, x: P, y: P, w: S, h: S) -> Self {
        Tag::new("image").xy(x, y).wh(w, h).href(loc)
    }
    pub fn ellipse<P: Display, S: Display>(cx: P, cy: P, rx: S, ry: S) -> Self {
        Tag::new("ellipse").cx(cx).cy(cy).rx(rx).ry(ry)
    }
    pub fn path<P: Display>(p: P) -> Self {
        Tag::new("path").d(p)
    }
    pub fn write<W: SvgWrite>(&self, w: &mut W) {
        w.write(&self.to_string());
    }
}

impl SvgArg for Tag {
    fn arg<T: Display>(mut self, k: &str, v: T) -> Self {
        self.args = self.args.arg(k, v);
        self
    }
    fn style<T: Display>(mut self, k: &str, v: T) -> Self {
        self.args = self.args.style(k, v);
        self
    }
    fn transform<T: Display>(mut self, k: &str, args: &[T]) -> Self {
        self.args = self.args.transform(k, args);
        self
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} {} />", self.name, self.args)
    }
}
