use std::io::{self, Cursor};

use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

use crate::itot::Tpkt;
use crate::result::{ConnectionError, Error, ProtocolError, Result};

#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(64 * 1024),
        }
    }
    pub async fn recv_tpkt(&mut self) -> Result<Option<Tpkt>> {
        loop {
            if let Some(tpkt) = self.parse_tpkt()? {
                return Ok(Some(tpkt));
            }
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(Error::Connection(ConnectionError::PeerReset));
                }
            }
        }
    }
    fn parse_tpkt(&mut self) -> Result<Option<Tpkt>> {
        let mut buf = Cursor::new(&self.buffer[..]);
        match Tpkt::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let tpkt = Tpkt::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(tpkt))
            }
            Err(Error::Protocol(ProtocolError::Incomplete)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn send_tpkt(&mut self, _tpkt: &Tpkt) -> io::Result<()> {
        self.stream.flush().await
    }
}

pub enum Request {
    Connect,
    Data,
    ExpeditedData,
    Disconnect,
}

pub enum Indication {
    Connect,
    Data,
    ExpeditedData,
    Disconnect,
    Unknown,
}

impl Indication {
    pub fn from_tpkt(_tpkt: Tpkt) -> Result<Indication> {
        Ok(Indication::Unknown)
    }
}

pub enum Response {
    Connect,
}

pub enum Confirmation {
    Connect,
}
