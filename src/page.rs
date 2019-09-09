use failure::Error;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::args::SvgArg;
use crate::tag::Tag;
use crate::write::{qcast, CDNum, SvgIO, SvgWrite};

pub struct Pages<'a, NT: CDNum, C> {
    cards: &'a [C],
    page_dims: Option<(NT, NT)>,
    grid_shape: Option<(usize, usize)>,
    card_size: Option<(NT, NT)>,
    init_defs: Box<Fn(&mut SvgWrite)>,
}

fn svg_no_init(_: &mut SvgWrite) {}

impl<'a, NT: CDNum, C: Card<NT>> Pages<'a, NT, C> {
    pub fn build(c: &'a [C]) -> Pages<'a, NT, C> {
        Pages {
            cards: c,
            page_dims: None,
            grid_shape: None,
            card_size: None,
            init_defs: Box::new(svg_no_init),
        }
    }

    pub fn write_page<W: SvgWrite>(&self, svg: &mut W, offset: usize) {
        let (pw, ph) = self.page_dims.unwrap_or((a4_width(), a4_height()));
        let (gw, gh) = self.grid_shape.unwrap_or((4, 4));
        let (cw, ch) = self.card_size.unwrap_or({
            let cw = (pw - qcast::<i32, NT>(40)) / qcast(gw);
            let ch = (ph - qcast::<i32, NT>(40)) / qcast(gh);
            (cw, ch)
        });

        let mw: NT = (pw - cw * qcast(gw)) / qcast(2);
        let mh: NT = (ph - ch * qcast(gh)) / qcast(2);

        let mut svg = Tag::start(svg, pw, ph);

        (self.init_defs)(&mut svg);

        let max = gw * gh;

        let cards = &self.cards[offset..];

        for (i, c) in cards.iter().enumerate() {
            if i == max {
                break;
            }
            let x: NT = qcast(i % gw);
            let y: NT = qcast(i / gw);
            let mut c_loc = Tag::g().translate(mw + x * cw, mh + y * ch).wrap(&mut svg);
            c.front(&mut c_loc, cw, ch);
        }
    }

    pub fn write_pages(&self, f_base: String) -> Result<Vec<String>, failure::Error> {
        let mut res = Vec::new();
        let (gw, gh) = self.grid_shape.unwrap_or((4, 4));

        let page_max = gw * gh;

        if self.cards.len() == 0 {
            return Ok(res);
        }

        for i in 0..((self.cards.len() - 1) / page_max) + 1 {
            let fname = format!("{}{}.svg", f_base, i);
            let w = File::create(&fname)?;
            let mut svg = SvgIO::new(w);
            self.write_page(&mut svg, i * page_max);
            res.push(fname);
        }
        Ok(res)
    }
}

pub fn a4_width<T: CDNum>() -> T {
    qcast(2480)
}
pub fn a4_height<T: CDNum>() -> T {
    qcast(3508)
}

pub trait Card<NT: CDNum>: Clone {
    fn front<S: SvgWrite>(&self, svg: &mut S, w: NT, h: NT);
}

pub fn page<W: Write, NT: CDNum, C: Card<NT>>(
    w: W,
    pw: NT,
    ph: NT,
    nw: usize,
    nh: usize,
    cards: &[C],
) {
    let mut svg = SvgIO::new(w);
    let mut svg = Tag::start(&mut svg, pw, ph);

    let mw: NT = pw / qcast(20);
    let mh: NT = ph / qcast(20);
    let max = nw * nh;
    let cw = (pw - qcast::<i32, NT>(2) * mw) / qcast(nw);
    let ch = (ph - qcast::<i32, NT>(2) * mh) / qcast(nh);

    for (i, c) in cards.iter().enumerate() {
        if i == max {
            break;
        }

        let x: NT = qcast(i % nw);
        let y: NT = qcast(i / nw);
        c.front(
            &mut Tag::g().translate(mw + x * cw, mh + y * ch).wrap(&mut svg),
            cw,
            ch,
        );
        //let mut svg = svg.g_translate(mw + x * cw, mh + y * ch);
        //c.front(&mut svg, cw, ch);
    }
}

pub fn page_a4<W: Write, NT: CDNum, C: Card<NT>>(w: W, nw: usize, nh: usize, cards: &[C]) {
    page(w, a4_width(), a4_height(), nw, nh, cards);
}

pub fn pages<NT: CDNum, C: Card<NT>, P: AsRef<Path>>(
    basepath: P,
    pw: NT,
    ph: NT,
    nw: usize,
    nh: usize,
    cards: &[C],
) -> Result<Vec<PathBuf>, Error> {
    let mut res = Vec::new();
    let total = nw * nh;

    let cpath: &Path = basepath.as_ref();
    let cname = OsString::from(cpath.file_name().unwrap_or(&OsStr::new("")));

    if cards.len() == 0 {
        return Ok(res);
    }

    for i in 0..((cards.len() - 1) / total) + 1 {
        let mut path = PathBuf::from(cpath.parent().unwrap_or(Path::new("")));
        let mut fname = cname.clone();
        fname.push(&format!("{}.svg", i));
        path.push(fname);
        let w = File::create(&path)?;
        page(w, pw, ph, nw, nh, &cards[i * total..]);
        res.push(path);
    }
    Ok(res)
}

pub fn pages_a4<NT: CDNum, C: Card<NT>, P: AsRef<Path>>(
    basepath: P,
    nw: usize,
    nh: usize,
    cards: &[C],
) -> Result<Vec<PathBuf>, Error> {
    pages(basepath, a4_width(), a4_height(), nw, nh, cards)
}

/// flip the items in groups 'w' big/wide
///
/// It is intended for printing the backs of cards, so flips them "horisontally"
/// it also inserts filler items to make sure the item on the last row is in the right place.
///
/// ```
/// use mksvg::page::page_flip;
/// let v = vec![1,2,3,4,5,6,7,8,9];
/// let v2 = page_flip(&v,4);
/// assert_eq!(v2,vec![4,3,2,1,8,7,6,5,1,1,1,9]);
/// ```
pub fn page_flip<T: Clone>(v: &Vec<T>, w: usize) -> Vec<T> {
    let mut res: Vec<T> = Vec::new();
    if v.len() == 0 {
        return res;
    }
    let blank = v[0].clone();
    let mut tmp = Vec::new();
    for elem in v {
        tmp.push(elem.clone());
        if tmp.len() == w {
            for e2 in tmp.into_iter().rev() {
                res.push(e2);
            }
            tmp = Vec::new();
        }
    }

    if tmp.len() > 0 {
        for _ in 0..w - tmp.len() {
            res.push(blank.clone());
        }
        for elem in tmp {
            res.push(elem);
        }
    }
    res
}

pub fn interlace<T: Clone>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut it_a = a.iter();
    let mut it_b = b.iter();
    let mut res: Vec<T> = Vec::new();
    loop {
        let mut done = 0;
        match it_a.next() {
            Some(t) => res.push(t.clone()),
            None => done += 1,
        }
        match it_b.next() {
            Some(t) => res.push(t.clone()),
            None => done += 1,
        }
        if done == 2 {
            return res;
        }
    }
}

pub fn unite_as_pdf<P: AsRef<Path>, Q: AsRef<Path>>(v: Vec<P>, fpath: Q) -> bool {
    let mut pdv: Vec<String> = Vec::new();
    for i in v {
        //get .pdf path
        let op = PathBuf::from(i.as_ref());
        let mut pp = op.clone();
        pp.set_extension("pdf");

        let pps = pp.to_str().unwrap_or("cc.pdf");
        print!("Creating : {}\n", pps);

        let _output = Command::new("inkscape")
            .arg(op)
            .arg(&format!("--export-pdf={}", pps))
            .output()
            .expect("Could not run process");

        pdv.push(pps.to_string());
    }

    pdv.push(fpath.as_ref().to_str().unwrap_or("pooyt4.pdf").to_string());
    print!("Combining\n");
    Command::new("pdfunite")
        .args(pdv)
        .output()
        .expect("could not unite the pdfs");

    true
}
