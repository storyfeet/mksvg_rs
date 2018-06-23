use args::{Args,SvgArg};
use std::io::Write;

use std::fmt::Display;
use num;

pub trait CDNum:num::Num+num::NumCast+ Copy + Display{}

impl<T:num::Num+num::NumCast+Copy+Display> CDNum for T {
}

pub fn qcast<A:CDNum,B:CDNum>(a:A)->B{
    num::NumCast::from(a).unwrap()
}


pub struct SvgW<W:Write> {
    w:W,
    d:i8,
}


impl<W:Write> SvgW<W> {
    pub fn new(w:W)->SvgW<W>{
        SvgW{
            w:w,
            d:0,
        }
    }
    fn pad(&self)->String{
        let mut res = "".to_string();
        for _i in 0..self.d {
            res.push_str("  ");
        }
        res
    }
}

impl<W:Write> SvgWrite for SvgW<W> {
    fn write(&mut self, s:&str){
        let ps = self.pad();
        write!(self.w,"{}{}\n",ps,s).unwrap();
    }
    fn inc_depth(&mut self,n:i8){
        self.d += n;
    }
}

pub trait SvgWrite{
    fn write(&mut self,s:&str);
    fn inc_depth(&mut self,n:i8);

    fn start<T:CDNum>(&mut self,w:T,h:T){
        self.write("<?xml version=\"1.0\" ?>");
        self.any_o("svg",&Args::new().w(w).h(h).arg("xmlns","http://www.w3.org/2000/svg").arg("xmlns:xlink","http://www.w3.org/1999/xlink"));
    }

    fn end(&mut self){
        self.inc_depth(-1);
        self.write("</svg>");
    }

    fn g(&mut self,args:Args){
        self.any_o("g",&args);
    }

    fn g_translate<T:CDNum>(&mut self, x:T,y:T){
        self.g(Args::new().translate(x,y));
    }

    fn g_rotate<T:CDNum>(&mut self,ang:T, x:T,y:T){
        self.g(Args::new().rotate(ang,x,y));
    }


    fn g_end(&mut self){
        self.inc_depth(-1);
        self.write("</g>");
    }

    fn any(&mut self,name:&str,args:&Args){
        self.write(&format!("<{} {} />",name,args));
    }

    fn any_o(&mut self,name:&str,args:&Args){
        self.write(&format!("<{} {} />",name,args));
        self.inc_depth(1);
    }


    fn rect<T:CDNum>(&mut self,x:T,y:T,w:T,h:T,args:Args){
        self.any("rect",&args.x(x).y(y).w(w).h(h));
    }

    fn ellipse<T:CDNum>(&mut self,cx:T,cy:T,rx:T,ry:T,args:Args){
        self.any("ellipse",&args.cx(cx).cy(cy).rx(rx).ry(ry));
    }

    fn text<T:CDNum>(&mut self,tx:&str,x:T,y:T,fs:T,args:Args){
        self.write(&format!("<text {} >{}</text>",
                            args.x(x).y(y).font_size(fs),tx));

    }

    fn bg_text<T:CDNum>(&mut self,tx:&str,x:T,y:T,fs:T,sw:T,scol:&str,args:Args){
        self.text(tx,x,y,fs,args.clone().stroke_width(sw).stroke(scol));
        self.text(tx,x,y,fs,args.stroke("none"));
    }

    fn bg_text_lines<T:CDNum>(&mut self,tx:&str,x:T,y:T,fs:T,dy:T,sw:T,scol:&str,args:Args){
        let p = args.font_size(fs);
        self.text_lines(tx,x,y,fs,dy,p.clone().stroke_width(sw).stroke(scol));
        self.text_lines(tx,x,y,fs,dy,p.stroke("none"));
    }

    fn text_lines<T:CDNum>(&mut self,tx:&str,x:T,y:T,fs:T,dy:T,args:Args){
        let lns = tx.split("\n"); 
        let mut ln_y:T = y;
        for ln in lns{
            self.text(&ln,x,ln_y,fs,args.clone().y(ln_y));
            ln_y = ln_y+  dy;
        }
    }

    fn img<T:Display>(&mut self,loc:&str,x:T,y:T,w:T,h:T){
        self.any("image",&Args::new().x(x).y(y).w(w).h(h).href(loc));
    }
}



