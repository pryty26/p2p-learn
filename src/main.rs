#![warn(clippy::pedantic)]

use anyhow::Result;
use anyhow::bail;
use derive_deftly::Deftly;
use pt_config::derive_deftly_template_FromDiscriminant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use std::net::SocketAddrV6;
const MAGIC_COOKIE: u32 = 0x2112_A442;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Deftly)]
#[derive_deftly(FromDiscriminant)]
pub enum StunReqType {
    Request = 0b00,
    Indication = 0b01,
    SuccessResponse = 0b10,
    ErrorResponse = 0b11,
}

#[tokio::main]
async fn main() -> Result<()> {
    let x = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut conn, addr) = x.accept().await?;
        tokio::spawn(async move {
            let mut stun_header = [0_u8; 20];
            conn.read_exact(&mut stun_header).await?;
            let msg_type = u16::from_be_bytes(stun_header[0..2].try_into()?);
            parse_header(msg_type)?;
            Ok::<(), anyhow::Error>(())
        });
    }
    Ok(())
}

fn parse_header(raw: u16) -> Result<(u8, u8)> {
    if raw & 0b1100_0000_0000_0000 != 0 {
        bail!("Invalid shit Fuck you idiot")
    }
    let m0_m3: u8 = (raw & 0b1111) as u8;
    let c0: u8 = (raw >> 4 & 1) as u8;
    let m4_m6: u8 = (raw >> 5 & 0b111) as u8;
    let c1 = (raw >> 8 & 0b1) as u8;
    let m7_m11 = (raw >> 9 & 0b11111) as u8;
    let c = c1 << 1 | c0;
    let m = m7_m11 << 7 | m4_m6 << 4 | m0_m3;

    Ok((m, c))
}

pub fn xor_addr(addr: SocketAddr) -> Result<()> {

    let x = [0_u8; 2];
    let y = [1_u8; 2];
    let z = ;
    match addr {
        SocketAddr::V4(addr) => {
            addr 
        },
        SocketAddrV6::V4(addr) => {}
    }
    Ok(())
}
// Bit position:
// 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// | 0| 0| M11 M10 M9 M8 M7 C1 M6 M5 M4 C0 M3 M2 M1 M0|
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+【--
