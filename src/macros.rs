//use crate::*;
//head tail style propery list processing.
#[macro_export]
macro_rules! svg_properties {
    //base case
    ($e:expr,) => {
        $e
    };
    //case with empty rhs
    ($e:expr, ($p:ident , ()),$($rest:tt)*) => (svg_properties!($e.$p(),$($rest)*));
    //case with bracketed properties
    ($e:expr, ($p:ident , ($($v:expr),*)),$($rest:tt)*) => (svg_properties!($e.$p($($v),*),$($rest)*));
    //standard case
    ($e:expr, ($p:ident , $v:expr) ,$($rest:tt)*) => (svg_properties!($e.$p($v),$($rest)*));
}

#[macro_export]
macro_rules! svg_w {
    //simple svg component
    ($wr:ident, ($nm:ident $($k:ident=$v:tt)*)) => {
        svg_properties!(Tag::new(stringify!($nm)), $(($k,$v),)*).write($wr);
    };
    //wrapping svg component
    ($wr:ident, ($nm:ident $($k:ident=$v:tt)* : $($child:tt)* ))=> {
        let mut nw = svg_properties!(Tag::new(stringify!($nm)), $(($k,$v),),*).wrap($wr);
        let nwp = &mut nw;
        $(
            svg_w!(nwp , $child);
        )*

        drop(nw);
    };
    ($wr:ident,( @if $b:expr => $($child:tt)*)) =>{
        if $b {
            $(
                svg_w!($wr , $child);
            )*
        }
    };
    ($wr:ident, (@for $p:pat in $i:expr => $($child:tt)*)) =>{
        for $p in $i  {
            $(
                svg_w!($wr , $child);
            )*
        }
    };

 /*   ($wr:ident,(|$n:ident|  $e:tt))=> {
        let f = <E>|$n:&mut dyn SvgWrite<Err=<E>>|$e;
        f($wr);
    };*/
    ($wr:ident,[$l:expr , $x:expr,$y:expr,$lh:expr ,$($k:ident=$v:tt)*])=>{
        svg_properties!( Text::lines($l,$x,$y,$lh) , $(($k,$v),)*).write($wr);
    };
    /*($wr:ident,$($ch:tt)+)=>{
        $(
            svg_w!($wr,$ch);
        )+
    };*/

}

#[cfg(test)]
mod test_macros {
    //    use super::*;
    use crate::*;

    #[test]
    fn test_svg_properties() {
        let mut s = String::new();
        let mut fw = SvgFmt::new(&mut s);
        let fwp = &mut fw;
        svg_w!(fwp,(rect w=5));
        drop(fw);
        assert_eq!(s, "<rect width=\"5\" />\n");
    }

    /*#[test]
    fn test_macro_does_a_thing() {
        let mut s = String::new();
        let mut fw = SvgFmt::new(&mut s);
        svg_w2! {
            fw, (rect x=32 y=0)
        };

        drop(fw);
        assert_eq!(s, "<help>");
    }*/

    #[test]
    fn test_wrapping_works() {
        let mut s = String::new();
        let mut fw = SvgFmt::new(&mut s);
        let fwp = &mut fw;
        svg_w! {fwp,
            ( g w=3:
                (rect x=3 y=2)
            )
        }
        drop(fw);
        assert_eq!(s, "<g width=\"3\" >\n  <rect x=\"3\" y=\"2\" />\n</g>\n");
    }

    #[test]
    fn test_multi_param_properties() {
        let mut s = String::new();
        let mut fw = SvgFmt::new(&mut s);
        let fwp = &mut fw;
        svg_w! {fwp,
            ( g translate=(3,4):
                (rect x=3 y=2)
            )
        }
        drop(fw);
        assert_eq!(
            s,
            "<g transform=\"translate(3,4) \" >\n  <rect x=\"3\" y=\"2\" />\n</g>\n"
        );
    }

    #[test]
    fn test_deep_wrapping() {
        let mut s = String::new();
        let mut fw = SvgFmt::new(&mut s);
        let fwp = &mut fw;
        svg_w! {fwp,
            ( g translate=(3,4):
                (g rotate=(15,24,45) :
                    (ellipse x=5 y=2)
                    (rect x=3 y=2)
                )
            )
        }
        drop(fw);
        assert_eq!(
            s,
            "<g transform=\"translate(3,4) \" >\n  <g transform=\"rotate(15,24,45) \" >\n    <ellipse x=\"5\" y=\"2\" />\n    <rect x=\"3\" y=\"2\" />\n  </g>\n</g>\n"
        );
    }

    #[test]
    fn test_loop() {
        let mut s = String::new();
        let mut fw = SvgFmt::new(&mut s);
        let fwp = &mut fw;
        svg_w! {fwp,
            (@for n in 0..4 =>
                (rect x=n)
            )
        }
        drop(fw);
        assert_eq!(
            s,
            "<rect x=\"0\" />\n<rect x=\"1\" />\n<rect x=\"2\" />\n<rect x=\"3\" />\n"
        );
    }

    #[test]
    fn test_macro_closure() {
        let mut s = String::new();
        let mut fw = SvgFmt::new(&mut s);
        let fwp = &mut fw;
        svg_w! {fwp,
            (text text_anchor="middle" :
                (|v| {v.write("hello")})
            )
        }
        drop(fw);
        assert_eq!(s, "<text text-anchor=\"middle\" >\n  hello\n</text>\n");
    }
}
