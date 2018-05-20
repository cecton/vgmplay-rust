#![feature(exclusive_range_pattern)]

extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

mod vgm;
use vgm::*;

#[allow(unused_must_use)]
fn main() {
    let file = std::env::args().skip(1).next();

    if file.is_none() {
        println!("missing file argument");
    }

    if let Err(err) = vgm2pcm(file.unwrap()) {
        println!("{:?}", err);
    }
}

fn vgm2pcm(path: String) -> bincode::Result<()> {
    let mut file = File::open(path)?;
    let header = vgm::Header::new(&mut file)?;

    if header.data_offset > 0 {
        file.seek(SeekFrom::Current(header.data_offset as i64))?;
        println!("seek to {}", header.data_offset);
    }

    loop {
        use vgm::Command::*;

        let pos = file.seek(SeekFrom::Current(0))?;

        if pos == header.loop_offset as u64 {
            println!("loop start")
        }
        if pos >= header.loop_samples as u64 {
            println!("loop ends")
        }

        let command =
            vgm::Command::new(&mut file).expect(&format!("position {} (0x{:x})", pos, pos));

        match command {
            End => break,
            _ => {}
        }
    }

    Ok(())
}
