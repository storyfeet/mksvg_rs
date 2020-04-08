use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::args::SvgArg;
use crate::tag::Tag;
use crate::write::{qcast, CDNum, SvgIO, SvgWrite};

pub struct Pages<'a, NT: CDNum> {
    flip: bool,
    page_dims: Option<(NT, NT)>,
    grid_shape: Option<(usize, usize)>,
    card_size: Option<(NT, NT)>,
    init_defs: Option<&'a dyn for<'r> Fn(&'r mut (dyn SvgWrite + 'r))>,
    //pc: std::marker::PhantomData<C>,
}

impl<'a, NT: CDNum> Pages<'a, NT> {
    pub fn build() -> Pages<'a, NT> {
        Pages {
            flip: false,
            page_dims: None,
            grid_shape: None,
            card_size: None,
            init_defs: None,
        }
    }

    pub fn flip(mut self) -> Self {
        self.flip = true;
        self
    }

    pub fn page_size(mut self, w: NT, h: NT) -> Self {
        self.page_dims = Some((w, h));
        self
    }

    pub fn grid_size(mut self, nw: usize, nh: usize) -> Self {
        self.grid_shape = Some((nw, nh));
        self
    }

    pub fn card_size(mut self, cw: NT, ch: NT) -> Self {
        self.card_size = Some((cw, ch));
        self
    }

    pub fn init_page(mut self, f: &'a dyn for<'r> Fn(&'r mut (dyn SvgWrite + 'r))) -> Self {
        self.init_defs = Some(f);
        self
    }

    pub fn write_page<W, I, C, F>(&mut self, svg: &mut W, it: &mut I, f: F) -> usize
    where
        W: SvgWrite,
        I: Iterator<Item = C>,
        F: Fn(&mut dyn SvgWrite, NT, NT, C),
    {
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

        if let Some(ref f) = self.init_defs {
            f(&mut svg);
        }

        let max = gw * gh;
        let mut i = 0;

        while let Some(c) = it.next() {
            let x: NT = if self.flip {
                qcast(gw - (i % gw))
            } else {
                qcast(i % gw)
            };
            let y: NT = qcast(i / gw);
            let mut c_loc = Tag::g().translate(mw + x * cw, mh + y * ch).wrap(&mut svg);
            f(&mut c_loc, cw, ch, c);

            i += 1;
            if i == max {
                return i;
            }
        }
        i
    }

    pub fn write_pages<S, I, F, C>(
        &mut self,
        f_base: S,
        it: &mut I,
        f: &F,
    ) -> Result<(usize, Vec<String>), failure::Error>
    where
        S: AsRef<str>,
        I: Iterator<Item = C>,
        F: Fn(&mut dyn SvgWrite, NT, NT, C),
    {
        let mut res = Vec::new();
        let mut tot_printed = 0;

        for i in 0.. {
            let fname = format!("{}{}.svg", f_base.as_ref(), i);
            let w = File::create(&fname)?;
            let mut svg = SvgIO::new(w);
            let printed = self.write_page(&mut svg, it, f);
            if printed == 0 {
                return Ok((tot_printed, res));
            }
            tot_printed += printed;
            res.push(fname);
        }
        Ok((tot_printed, res))
    }
}

pub fn a4_width<T: CDNum>() -> T {
    qcast(2480)
}
pub fn a4_height<T: CDNum>() -> T {
    qcast(3508)
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

#[cfg(test)]
mod page_test {
    use super::*;
    use crate::write::SvgFmt;
    pub struct NumCard(i32);
    fn draw_card(wr: &mut dyn SvgWrite, _w: i32, _h: i32, c: NumCard) {
        wr.write(&c.0.to_string());
    }
    #[test]
    pub fn test_closure_works_with_page_write() {
        let mut s = String::new();
        let mut svg = SvgFmt::new(&mut s);
        let v = vec![NumCard(4050)];
        Pages::build()
            .init_page(&|ref mut w| Tag::rect(0, 0, 5, 5).write(w))
            .write_page(&mut svg, &mut v.into_iter(), draw_card);

        assert_eq!(s, "<?xml version=\"1.0\" ?>\n<svg width=\"2480\" height=\"3508\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" >\n  <rect x=\"0\" y=\"0\" width=\"5\" height=\"5\" />\n  <g transform=\"translate(20,20) \" >\n    4050\n  </g>\n</svg>\n");
    }
}
