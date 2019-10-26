pub use arror_derive::Arror;
use failure::{Error, Fail};


#[derive(Fail, Debug)]
pub enum Arror {
  #[fail(display="user error")]
  User(#[fail(cause)] Error, bool),

  #[fail(display="internal error")]
  Internal(#[fail(cause)] Error, bool),

  #[fail(display="runtime error")]
  Runtime(#[fail(cause)] Error, bool),

  #[fail(display="evil error")]
  Evil(#[fail(cause)] Error, bool),

}


