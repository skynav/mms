#![allow(unreachable_code)]

use std::fs::File;

use log::{info};
use simplelog::*;
use tokio::net::{TcpListener,TcpStream};

use mms::{connection,server};
use mms::itot::Tpkt;
use mms::Result;

#[tokio::main]
async fn main() -> Result<()> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("play.log").unwrap()),
        ]
    ).unwrap();
    info!("start example");
    let port = server::DEFAULT_PORT;
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
    info!("end example");
    Ok(())
}

async fn process(socket: TcpStream) {
    use connection::{Connection,Indication};
    let mut connection = Connection::new(socket);
    while let Some(tpkt) = connection.recv_tpkt().await.unwrap() {
        let response = match Indication::from_tpkt(tpkt).unwrap() {
            Indication::Connect => Tpkt::Empty,
            Indication::Data => Tpkt::Empty,
            Indication::Disconnect => Tpkt::Empty,
            _ => Tpkt::Empty
        };
        connection.send_tpkt(&response).await.unwrap();
    }
}
