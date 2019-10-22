pub use arror_derive::Arror;

use failure::Error;
use failure::Fail;


#[derive(Fail, Debug)]
pub enum Arror {
  #[fail(display="user error")]
  User(#[fail(cause)] Error),

  #[fail(display="internal error")]
  Internal(#[fail(cause)] Error),

  #[fail(display="runtime error")]
  Runtime(#[fail(cause)] Error),

  #[fail(display="evil error")]
  Evil(#[fail(cause)] Error),
}


