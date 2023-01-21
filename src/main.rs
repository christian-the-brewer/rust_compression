extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    //check for required args
    if args().len() != 3 {
        eprintln!("Usage: 'source' 'target'");
        return;
    }
    //take in file and destination for encoding
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    //encoder
    let mut encoder = GzEncoder::new(output, Compression::default());
    //for tracking time
    let start = Instant::now();
    //encoding input with encoder
    copy(&mut input, &mut encoder).unwrap();
    //saving as output
    let output = encoder.finish().unwrap();
    //print info to user
    println!(
        "Uncompressed size: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Compressed size: {:?}", output.metadata().unwrap().len());
    println!("Time elapsed: {:?}", start.elapsed());
}
