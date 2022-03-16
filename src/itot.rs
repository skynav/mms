use std::fmt::{self, Display};
use std::io::Cursor;

use crate::cotp::Tpdu;
use crate::result::Result;

#[derive(Clone, Debug)]
pub enum Tpkt {
    Tpdu(Tpdu),
    Error(String),
    Empty,
}

impl Tpkt {
    pub fn check(_src: &mut Cursor<&[u8]>) -> Result<()> {
        Ok(())
    }
    pub fn parse(_src: &mut Cursor<&[u8]>) -> Result<Tpkt> {
        Ok(Tpkt::Empty)
    }
}

impl Display for Tpkt {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "tpkt")
    }
}
