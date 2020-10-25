
// // -------------------------------CLI Parameters-------------------------------
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {

    /// Path to the file to read
    #[structopt(
        short="i", 
        long = "--infile",
        parse(from_os_str)
    )]
        infile: std::path::PathBuf,
}
// // -------------------------------Stop CLI Parameters-------------------------------

// // -------------------------------BAM file-------------------------------
// extern crate bam;
// use std::io;
// use bam::RecordWriter;

// fn main() {
//     let args = Cli::from_args();
//     let filestr = std::fs::read_to_string(&args.infile)
//     .expect("could not read file");
//     let mut reader = bam::IndexedReader::from_path(filestr).unwrap();

//     // Do Stuff
//     // Documentation Example below: https://docs.rs/bam/0.1.1/bam/
//     let output = io::BufWriter::new(io::stdout());
//     let mut writer = bam::BamWriter::build()
//         .write_header(false)
//         .from_stream(output, reader.header().clone()).unwrap();
//     for record in reader.fetch(&bam::Region::new(2, 600_000, 700_000)).unwrap() {
//         let record = record.unwrap();
//         writer.write(&record).unwrap();
//     }
// }
// // -------------------------------Stop BAM file-------------------------------

// // -------------------------------SAM file-------------------------------
// extern crate bam;

// use std::io;
// use bam::sam::SamReader;
 
// fn main() {
//     let args = Cli::from_args();
//     let filestr = std::fs::read_to_string(&args.infile)
//     .expect("could not read file");
//     let reader = SamReader::from_path("in.sam").unwrap();

//     // Do something.
//     // Documentation Example: https://docs.rs/bam/0.1.1/bam/sam/struct.SamReader.html
//     let mut reader = bam::SamReader::from_path("in.sam").unwrap();
//     for record in reader {
//         let record = record.unwrap();
//     }
// }

// // -------------------------------Stop SAM file-------------------------------

// // -------------------------------FASTQ file-------------------------------
// extern crate seq_io;

// use seq_io::fastq::{Reader,Record};

// fn main() {
//     let args = Cli::from_args();
//     let filestr = std::fs::read_to_string(&args.infile)
//     .expect("could not read file");
//     let mut reader = Reader::from_path(filestr).unwrap();

//     // Do something.
//     // Documentation Example: https://docs.rs/seq_io/0.3.1/seq_io/
//     while let Some(record) = reader.next() {
//         let record = record.expect("Error reading record");
//         println!("{}", record.id().unwrap());
//     }
// }

// // -------------------------------Stop FASTQ file-------------------------------

// // -------------------------------FASTA file-------------------------------
// extern crate seq_io;

// use seq_io::fasta::{Reader,Record};

// fn main() {
//     let args = Cli::from_args();
//     let filestr = std::fs::read_to_string(&args.infile)
//     .expect("could not read file");
//     let mut reader = Reader::from_path(filestr).unwrap();

//     // Do something.
//     // Documentation Example: https://docs.rs/seq_io/0.3.1/seq_io/
//     while let Some(record) = reader.next() {
//         let record = record.expect("Error reading record");
//         println!("{}", record.id().unwrap());
//     }
// }
// // -------------------------------Stop FASTA file-------------------------------

// // -------------------------------GFF file-------------------------------
// use std::io;
// use bio::io::gff;

// fn main() {
//     let args = Cli::from_args();
//     let filestr = std::fs::read_to_string(&args.infile)
//     .expect("could not read file");
//     let mut reader = gff::Reader::from_file(filestr, gff::GffType::GFF3).unwrap();

// //     // Do something.
// //     // Documentation Example: 

//     let mut writer = gff::Writer::new(vec![], gff::GffType::GFF3);
//     for record in reader.records() {
//         let rec = record.ok().expect("Error reading record.");
//         println!("{}", rec.seqname());
//         writer.write(&rec).ok().expect("Error writing record.");
//     }
// }
// // -------------------------------Stop GFF file-------------------------------


// // -------------------------------Bigwig file-------------------------------
// use bigtools::bigwigread::BigWigRead;

// fn main() {
//     let args = Cli::from_args();
//     let filestr = std::fs::read_to_string(&args.infile)
//     .expect("could not read file");
//     let mut reader = BigWigRead::from_file_and_attach(&filestr).unwrap();

    // //     // Do something.
    // //     // Documentation Example: https://docs.rs/crate/bigtools/0.1.2
    // let chr1 = reader.get_interval("chr1", 0, 10000).unwrap();
    // for interval in chr1 {
    //     println!("{:?}", interval);
    // }
// }

// // -------------------------------Stop Bigwig file-------------------------------
// for other 'big' files, use same structure as above, only edit
// use bigtools:: and bedparser
// // -------------------------------Stop Bigwig file-------------------------------


// // -------------------------------Tabix file-------------------------------
// use bgzip::{BGZFReader, BGZFError};
// use std::io::{self, BufRead};
// use std::fs;

// fn main() {
//     let args = Cli::from_args();
//     let filestr = std::fs::read_to_string(&args.infile)
//     .expect("could not read file");
//     let mut reader = BGZFReader::new(filestr);
//     let mut line = String::new();
// }
// // -------------------------------Stop Tabix file-------------------------------
