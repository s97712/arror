extern crate arror;
use failure::{Fail, Error, AsFail};
use arror::Arror;


#[derive(Fail, Debug, Arror)]
#[arror(Internal)]
pub enum TestError {
  #[fail(display="test default")]
  TestDefault,

  #[fail(display="test override")]
  #[arror(Evil)]
  TestOverride,

  #[fail(display="test other")]
  Other(#[fail(cause)] Error),
}

fn cause_error (err: TestError) -> Result<(), Arror> {
  Err(err)?;
  Ok(())
}


#[test]
fn test_default() {
  let err = cause_error(TestError::TestDefault);

  match err {
    Err(Arror::Internal(err)) => {
      assert_eq!(err.as_fail().to_string(), "test default")
    },
    _ => {
      unreachable!()
    }
  };


}

#[test]
fn test_override() {
  let err = cause_error(TestError::TestOverride);

  match err {
    Err(Arror::Evil(err)) => {
      assert_eq!(err.as_fail().to_string(), "test override")
    },
    _ => {
      unreachable!()
    }
  };
}
