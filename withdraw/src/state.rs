
pub trait State {
  fn state<'a>(&self) -> &'a str;

}

pub(super) struct Initiate { }
impl State for Initiate {
  fn state<'a>(&self) -> &'a str {
      "initiated"
  }
}

pub(super) struct Submit { }
impl State for Submit {
    fn state<'a>(&self) -> &'a str {
        "submitted"
    }
}

pub(super) struct Accepted { }
impl State for Accepted {
  fn state<'a>(&self) -> &'a str {
      "accepted"
  }
}

pub(super) struct Rejected { }
impl State for Rejected {
    fn state<'a>(&self) -> &'a str {
        "rejected"
    }
}


pub(super) struct Processing { }
impl State for Processing {
    fn state<'a>(&self) -> &'a str {
        "processing"
    }
}

pub(super) struct Succeed { }
impl State for Succeed {
    fn state<'a>(&self) -> &'a str {
        "succeed"
    }
}

pub(super) struct Failed { }
impl State for Failed {
   fn state<'a>(&self) -> &'a str {
       "rejected"
   } 
}
