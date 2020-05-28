use crate::args::{Args, SvgArg};
use crate::write::{SvgWrite, TransWrap};
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    name: &'static str,
    args: Args,
}

impl Tag {
    /// Use this function to wrap the first svg tag, as it also writes the namespace to the top of
    /// the doc
    pub fn start<'a, W: SvgWrite<Err = E>, T: Display, E>(
        wr: &'a mut W,
        w: T,
        h: T,
    ) -> Result<TransWrap<'a, E>, E> {
        wr.write(r#"<?xml version="1.0" ?>"#)?;
        Ok(Tag::svg(w, h).wrap(wr))
    }

    fn svg<T: Display>(w: T, h: T) -> Self {
        Tag::new("svg")
            .w(w)
            .h(h)
            .arg("xmlns", "http://www.w3.org/2000/svg")
            .arg("xmlns:xlink", "http://www.w3.org/1999/xlink")
    }
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

    pub fn g() -> Self {
        Tag::new("g")
    }

    pub fn defs() -> Self {
        Tag::new("defs")
    }

    pub fn use_tag<T: Display>(href: T) -> Self {
        Tag::new("use").href(href)
    }

    pub fn clip_path() -> Self {
        Tag::new("clipPath")
    }

    pub fn write<W: SvgWrite<Err = E>, E>(&self, w: &mut W) -> Result<(), E> {
        w.write(&self.to_string())
    }

    pub fn wrap<'a, W: SvgWrite<Err = E>, E>(&self, w: &'a mut W) -> TransWrap<'a, E> {
        TransWrap::new(
            w,
            &format!("<{} {}>", self.name, self.args),
            &format!("</{}>", self.name),
        )
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
        write!(f, "<{} {}/>", self.name, self.args)
    }
}
