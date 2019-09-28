//use crate::*;

#[macro_export]
macro_rules! svg_w {
    ($wr:ident, $tp:ident $($k:tt = $v:tt)*)=> {
        Tag::new(stringify!($tp))$(.$k($v))*.write(&mut $wr);
    };
}

#[cfg(test)]
mod test_macros {
    use super::*;
    use crate::*;
    #[test]
    fn test_macro_does_a_thing() {
        let mut s = String::new();
        let mut fw = SvgFmt::new(&mut s);
        svg_w! {
            fw, rect x=32 y=0
                
        };

        drop(fw);
        assert_eq!(s,"<help>");

    }
}
