use failure::*;
//use failure_derive::*;

#[derive(Fail, Debug)]
pub enum PageError<C: Fail> {
    #[fail(display = "Card Error{}", 0)]
    CardError(C),
    #[fail(display = "Message{}", 0)]
    SMess(&'static str),
}
