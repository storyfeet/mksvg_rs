extern crate num;
pub mod args;
pub mod write;
pub mod page;
pub mod text;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
