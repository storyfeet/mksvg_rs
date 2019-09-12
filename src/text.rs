//! A utility for wrapping text, and allowing users to insert new lines fromconfig files etc.
//! This primarily exists to enable the text_lines method in "write.rs" to print as desired.
//!

use crate::args::{Args, SvgArg};
use crate::write::{qcast, CDNum, SvgWrite};
use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Clone, Debug, PartialEq)]
pub struct Text<C: CDNum> {
    ss: Vec<String>,
    args: Args,
    back: Option<(C, String)>,
    x: C,
    y: C,
    line_height: C,
    font_size_set: bool,
}

impl<C: CDNum> Text<C> {
    pub fn new<S: AsRef<str>>(s: S, x: C, y: C, lh: C) -> Self {
        let s: &str = s.as_ref();
        Self::lines(s.split("\n"), x, y, lh)
    }

    pub fn lines<I, S>(it: I, x: C, y: C, lh: C) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Text {
            ss: it.into_iter().map(|v| v.as_ref().to_string()).collect(),
            args: Args::new(),
            back: None,
            x,
            y,
            line_height: lh,
            font_size_set: false,
        }
    }

    pub fn wrap(mut self, n: usize) -> Self {
        let mut res = Vec::new();
        for s in self.ss {
            for r in wrap(&s, n) {
                res.push(r);
            }
        }
        self.ss = res;
        self
    }

    pub fn v_center(mut self) -> Self {
        self.y = self.y - self.line_height * qcast(self.ss.len() as f64 / 2.);
        self
    }

    pub fn v_base(mut self) -> Self {
        self.y = self.y - self.line_height * qcast(self.ss.len());
        self
    }
    pub fn bg<S: AsRef<str>>(mut self, sw: C, col: S) -> Self {
        self.back = Some((sw, col.as_ref().to_string()));
        self
    }

    pub fn write<S: SvgWrite>(&self, s: &mut S) {
        for (n, l) in self.ss.iter().enumerate() {
            let mut a = self.args.clone();
            if !self.font_size_set {
                a = a.font_size(self.line_height)
            }
            a = a.xy(self.x, self.y + self.line_height * qcast(n));
            if let Some((w, ref col)) = self.back {
                let a2 = a.clone().stroke_width(w).stroke(col);
                s.write(&format!("<text {}>{}</text>", a2, l));
            }
            s.write(&format!("<text {}>{}</text>", a, l));
        }
    }
}

impl<C: CDNum> Display for Text<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<C: CDNum + Debug> SvgArg for Text<C> {
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

    fn font_size<T: Display>(mut self, t: T) -> Self {
        self.font_size_set = true;
        self.args = self.args.font_size(t);
        self
    }
}

/// convert escaped characters to their standard response.
///
/// ```
/// use mksvg::text::escapes;
/// assert_eq!(&escapes("he\\\\n \\n\\t\\p") ,"he\\n \n\tp");
/// ```
pub fn escapes(s: &str) -> String {
    let mut res = String::new();
    let mut esc = false;
    for c in s.chars() {
        if esc {
            match c {
                't' => res.push('\t'),
                'n' => res.push('\n'),
                'r' => res.push('\r'),
                _ => res.push(c),
            }
            esc = false;
            continue;
        }
        match c {
            '\\' => esc = true,
            _ => res.push(c),
        }
    }
    res
}

/// wrap text to width mx, by adding \n where needed.
///
/// ```
/// use mksvg::text::wrap_nl;
/// assert_eq!(&wrap_nl("he-llo hello-people",5),"he-\nllo\nhello-\npeople");
/// ```
pub fn wrap_nl(s: &str, mx: usize) -> String {
    wrap(s, mx).join("\n")
}

/// wrap text to a max line lencth of mx, returning a Vec of String
///
/// ```
/// use mksvg::text::wrap;
/// assert_eq!(&wrap("hello everybody",6),&["hello","everyb-","ody"]);
/// assert_eq!(&wrap("hi to the people i know",6),&["hi to","the","people","i know"]);
/// ```
pub fn wrap(s: &str, mx: usize) -> Vec<String> {
    let mut cword = String::new();
    let mut cline = String::new();
    let mut res: Vec<String> = Vec::new();

    for c in s.chars() {
        if cline.len() + cword.len() > mx {
            if cline.len() == 0 {
                cline.push_str(&cword[..mx]);
                cline.push('-');
                cword = String::from(&cword[mx..]);
            } else {
                cword = cword[1..].to_string();
            }

            res.push(cline);
            cline = "".to_string();
        }
        match c {
            '\n' => {
                cline.push_str(&cword);
                cword.clear();
                res.push(cline);
                cline = "".to_string();
            }
            '-' => {
                cline.push_str(&cword);
                cline.push('-');
                cword = String::from(" ");
            }
            ' ' => {
                cline.push_str(&cword);
                cword = String::from(" ");
            }
            _ => {
                cword.push(c);
            }
        }
    }
    cline.push_str(&cword);
    res.push(cline);
    res
}
