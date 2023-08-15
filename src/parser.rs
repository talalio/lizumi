use std::env;
use std::str::FromStr;
use std::process::exit;
use crate::help::*;

#[allow(dead_code)]
#[derive(Debug)]
pub enum OperationT {
	Help = 1,
	Listen,
	Generate,
}

// i know
#[derive(Debug)]
pub struct Operation {
	pub optype: OperationT,
	pub lhost: String,
	pub lport: u16
}

pub fn parse() -> Option<Operation> {
	let args = env::args().collect::<Vec<String>>();

	if args.len() < 2 {
		println!("{HELP_MENU}");
		exit(0);
	}

	let op = args[1].as_str();
	if op == "help" {
		println!("{HELP_MENU}");
		exit(0);
	} else if op == "gen" || op == "listen" {
		if (args.len() - 1) != 3 {
			println!("\tlizumi {} LHOST LPORT", (if op == "gen" {"gen"} else {"listen"}));
			exit(0);
		}

		let lhost = args[2].clone();
		let lport = u16::from_str(args[3].as_str()).expect("[err] invalid port number!");

		return Some(Operation {
						optype: (if op == "gen" {OperationT::Generate} else {OperationT::Listen}),
						lhost: lhost,
						lport: lport
					}
				);

	} else {
		println!("{HELP_MENU}");
	}

	None
}
