extern crate arror;
use failure::{Fail, Error, AsFail};
use arror::{Arror, ArrorKind};


#[derive(Fail, Debug, Arror)]
#[arror(Internal)]
pub enum TestError {
  #[fail(display="test default")]
  TestDefault,

  #[fail(display="test override")]
  #[arror(Evil, abort)]
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
    Err(arror) => {
      assert_eq!(arror.to_string(), "Internal");
      assert_eq!(arror.abort(), false);
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
    Err(arror) => {
      assert_eq!(arror.to_string(), "Evil");
      assert_eq!(arror.abort(), true);
    },
    _ => {
      unreachable!()
    }
  };
}
