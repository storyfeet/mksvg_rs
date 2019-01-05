//! path exists to make paths much easier to write svg paths
//! while you can make the path anywey you like and send that as the argument, it may be
//! simpler to create it like below:
//!
//! ```
//! use mksvg::*;
//! let mut s = "".to_string();
//! {
//!     let mut svg = SvgFmt::new(&mut s);
//!     // ... normally do svg start
//!     svg.path(PathD::rel().m(5,5).l(10,10),Args::new().stroke("black"));
//!     // ... normally do svg end
//! }
//!
//! assert_eq!("<path d=\"m 5 5 l 10 10 \" style=\"stroke:black;\" />\n" 
//!             ,&s);
//!
//! ```


use std::fmt;
use std::fmt::{Display,Formatter};

#[derive(Clone)]
pub struct PNode{
    tp:char,
    vals:String,
}


#[derive(Clone)]
pub struct PathD {
    items:Vec<PNode>,
    rel:bool,
}

impl Display for PathD {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut res = "".to_string();
        for a in &self.items {
            res.push_str(&format!("{} {} ",a.tp,a.vals));
        }
        write!(f, "{}",res)
    }
}

impl PathD {
    pub fn rel()->Self{
        PathD{
            items:Vec::new(),
            rel:true,
        }
    }
    pub fn abs()->Self{
        PathD{
            items:Vec::new(),
            rel:false,
        }
    }

    pub fn set_rel(mut self,b :bool)->Self{
        self.rel = b;
        self
    }

    ///add will make sure the option is case matched to the current relative/absolute state
    pub fn add(mut self,tp:char,vals:&str)->Self{
        match self.rel {
            true=>self.items.push(PNode{tp:tp.to_ascii_lowercase(),vals:vals.to_string()}),
            false=>self.items.push(PNode{tp:tp.to_ascii_uppercase(),vals:vals.to_string()}),
        }
        self
    }

    pub fn m<T:Display>(self,x:T,y:T)->Self{
        self.add('m',&format!("{} {}",x,y) )
    }

    
    pub fn l<T:Display>(self,x:T,y:T)->Self{
        self.add('l',&format!("{} {}",x,y) )
    }

    pub fn h<T:Display>(self,x:T)->Self{
        self.add('h',&format!("{}",x))
    }
    pub fn v<T:Display>(self,y:T)->Self{
        self.add('v',&format!("{}",y))
    }

    pub fn z(self)->Self{
        self.add('z',"")
    }
   
    pub fn c<T:Display>(self,cx1:T,cy1:T,cx2:T,cy2:T,x:T,y:T)->Self{
        self.add('c',&format!("{} {} {} {} {} {}",cx1,cy1,cx2,cy2,x,y))
    }

    pub fn s<T:Display>(self,cx:T,cy:T,x:T,y:T)->Self{
        self.add('s',&format!("{} {} {} {}",cx,cy,x,y))
    }


    pub fn q<T:Display>(self,cx:T,cy:T,x:T,y:T)->Self{
        self.add('q',&format!("{} {} {} {}",cx,cy,x,y))
    }

    
    pub fn t<T:Display>(self,x:T,y:T)->Self{
        self.add('t',&format!("{} {}",x,y) )
    }

    pub fn a<T:Display>(self,rx:T,ry:T,large:bool,clockwise:bool,x:T,y:T)->Self{
        self.add('a',&format!("{} {} {} {} {} {}",rx,ry,large as i8, clockwise as i8 ,x,y))
    }



}
