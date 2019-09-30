//use crate::*;
//head tail style propery list processing.
#[macro_export]
macro_rules! svg_properties {
    //base case
    ($e:expr,) => {
        $e
    };
    //case with bracketed properties
    ($e:expr, ($p:ident , ($($v:expr),*)),$($rest:tt)*) => (svg_properties!($e.$p($($v),*),$($rest)*));
    //standard case
    ($e:expr, ($p:ident , $v:expr) ,$($rest:tt)*) => (svg_properties!($e.$p($v),$($rest)*));
}

#[macro_export]
macro_rules! svg_w {
    //simple svg component
    ($wr:ident, ($nm:ident $($k:ident=$v:tt)*)) => {
        svg_properties!(Tag::new(stringify!($nm)), $(($k,$v),)*).write(&mut $wr);
    };
    //wrapping svg component
    ($wr:ident, ($nm:ident $($k:ident=$v:tt)* : $($child:tt)* ))=> {
        let mut nw = svg_properties!(Tag::new(stringify!($nm)), $(($k,$v),),*).wrap(&mut $wr);
        $(
            svg_w!(nw , $child);
        )*

        drop(nw);

    };
}

#[cfg(test)]
mod test_macros {
    use super::*;
    use crate::*;

    #[test]
    fn test_svg_properties() {
        let mut s = String::new();
        let mut fw = SvgFmt::new(&mut s);
        svg_w!(fw,(rect w=5));
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
        svg_w! {fw,
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
        svg_w! {fw,
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
}
