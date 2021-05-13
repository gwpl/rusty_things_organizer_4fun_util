use std::fs;
use std::io;
use std::io::prelude::*;
//use std::io::prelude;
use std::vec::Vec;

pub fn reverse_inplace(b: &mut [u8], len: usize){
    for i in  0..(len/2) {
        b.swap(i,len-i-1);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("This program reverse each 8 bytes of file, so it's considered utf-8 unsafe, but reversable");
    let mut f = fs::OpenOptions::new().read(true).write(true).open("examples/test_reverse.txt")?;
    let mut buffer: [u8; 8] = [0; 8];
    let mut result: Vec<u8> = Vec::new();
    result.reserve_exact(f.metadata().unwrap().len() as usize);

    loop {
        let readlen = f.read(&mut buffer)?;
        if readlen == 0 {
            break;
        }
        reverse_inplace(&mut buffer, readlen);
        for b in buffer.iter() {
            result.push(*b);
        }
    }

    eprintln!("After 8bytes reverse: \n{}", String::from_utf8(result.clone())?);

    f.seek(io::SeekFrom::Start(0))?;
    eprintln!("result.len() = {}", result.len() as u64);
    f.set_len(result.len() as u64)?;
    f.write(&result)?;

    Ok(())
}
