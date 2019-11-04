pub use arror_derive::Arror;
use failure::{Fail, Causes};
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum ArrorKind {
  User,
  Internal,
  Runtime,
  Evil,
}


pub struct ArrorInner<F: Fail + ?Sized> {
  abort: bool,
  kind: ArrorKind,
  cause: F
}

pub struct Arror {
  inner: Box<ArrorInner<dyn Fail>>
}

impl Arror {
  pub fn new(abort: bool, kind:ArrorKind, cause: impl Fail) -> Arror {
    Arror {
      inner: Box::new(ArrorInner {
        abort,
        kind,
        cause
      })
    }
  }
  pub fn abort(&self) -> bool {
    self.inner.abort
  }

  pub fn kind(&self) -> ArrorKind {
    self.inner.kind
  }

  pub fn as_fail(&self) -> &dyn Fail {
    &self.inner.cause
  }

  pub fn trace_msg(&self) -> String {
    let mut causes = self.iter_chain();
    let mut msg = String::new();

    while let Some(err) = causes.next() {
      let name = err.name().unwrap_or("unnamed");

      let pos = match err.at() {
        Some(pos) => pos.to_string(),
        None => "...".to_owned()
      };

      msg = msg + &format!("\n\t [{}] {} at {}", name, &err.to_string(), &pos);

    }
    msg
  }
}

impl Arror {
  pub fn user<F: Fail>(abort: bool, cause: F) -> Arror {
    Arror::new(abort, ArrorKind::User, cause)
  }
  pub fn internal<F: Fail>(abort: bool, cause: F) -> Arror {
    Arror::new(abort, ArrorKind::Internal, cause)
  }
  pub fn runtime<F: Fail>(abort: bool, cause: F) -> Arror {
    Arror::new(abort, ArrorKind::Runtime, cause)
  }
  pub fn evil<F: Fail>(abort: bool, cause: F) -> Arror {
    Arror::new(abort, ArrorKind::Evil, cause)
  }
}

impl Arror {
  pub fn name(&self) -> Option<&str> {
    self.as_fail().name()
  }
  pub fn iter_causes(&self) -> Causes {
    self.as_fail().iter_causes()
  }
  pub fn iter_chain(&self) -> Causes {
    self.as_fail().iter_chain()
  }
  pub fn find_root_cause(&self) -> &dyn Fail {
    self.as_fail().find_root_cause()
  }
}

impl fmt::Display for Arror {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.inner.kind)
  }
}

