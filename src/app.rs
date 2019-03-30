use crate::builder::Builder;
use itertools::Itertools;
use md4::{Digest, Md4};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
#[structopt(name = "md4rip", about = "A MD4 Collision Generator.")]
pub struct Opt {
    /// The input file to use
    #[structopt(name = "INPUT", parse(try_from_os_str = "parse_existing_file"))]
    input: PathBuf,
    /// The collision's starting position
    #[structopt(name = "OFFSET")]
    offset: u64,
    #[structopt(name = "OUTPUT1", parse(from_os_str))]
    /// Path for output file 1
    output1: PathBuf,
    #[structopt(name = "OUTPUT2", parse(from_os_str))]
    /// Path for output file 2
    output2: PathBuf,
}

fn parse_existing_file(s: &OsStr) -> Result<PathBuf, OsString> {
    let p = PathBuf::from(s);
    if p.exists() {
        Ok(p)
    } else {
        Err(OsString::from("input file is not valid"))
    }
}

pub struct App {}

impl App {
    fn write_output(output: &PathBuf, input: &PathBuf, offset: u64, padding: &[u8], m: &[u8]) {
        let mut input_file = File::open(input)
            .unwrap_or_else(|_| panic!("failed to open input file {}", input.display()));

        let mut output_file = File::create(output)
            .unwrap_or_else(|_| panic!("failed to create output file {}", output.display()));

        // Copy input file to output file
        std::io::copy(&mut input_file, &mut output_file)
            .unwrap_or_else(|_| panic!("failed to copy input to output file {}", output.display()));

        // Seek output file to offset
        output_file
            .seek(SeekFrom::Start(offset))
            .unwrap_or_else(|_| panic!("seek to offset failed: file {}", output.display()));

        // Write padding
        output_file.write_all(padding).unwrap_or_else(|_| {
            panic!(
                "failed to write padding to output file {}",
                output.display()
            )
        });

        // Write message
        output_file.write_all(m).unwrap_or_else(|_| {
            panic!(
                "failed to write message to output file {}",
                output.display()
            )
        });
    }

    fn md4sum(path: &PathBuf) -> String {
        let file =
            File::open(path).unwrap_or_else(|_| panic!("failed to open file {}", path.display()));
        let mut reader = BufReader::new(file);

        let mut hasher = Md4::new();
        std::io::copy(&mut reader, &mut hasher).unwrap();

        format!("{:02x}", hasher.result().iter().format(""))
    }

    pub fn run(opt: Opt) {
        // Take input file
        let file = File::open(&opt.input).expect("failed to open input file");
        let len = file
            .metadata()
            .expect("failed to get input file metadata")
            .len();
        let limit = opt.offset;
        if limit > len {
            println!("======= ERROR: offset is larger than file size");
            return;
        }
        let mut reader = BufReader::new(file).take(limit);

        // Feed prefix into builder
        let mut builder = Builder::new();
        std::io::copy(&mut reader, &mut builder).unwrap();

        // Build
        match builder.build() {
            Ok((padding, m1, m2)) => {
                // Print info
                println!("=> Collision info");
                println!("Created collision starting at byte offset {}", limit);
                println!("Padding length: {} bytes", padding.len());
                if !padding.is_empty() {
                    println!("Padding: {:02x}", padding.iter().format(""));
                }
                println!("Message1: {:02x}", m1.iter().format(""));
                println!("Message2: {:02x}", m2.iter().format(""));

                // Write to output
                App::write_output(&opt.output1, &opt.input, opt.offset, &padding, &m1);
                App::write_output(&opt.output2, &opt.input, opt.offset, &padding, &m2);

                // Print md4sum
                println!("=> Output file:");
                let md4sum1 = App::md4sum(&opt.output1);
                let md4sum2 = App::md4sum(&opt.output2);

                println!(
                    "MD4Sum for {}: {}",
                    opt.output1.display(),
                    App::md4sum(&opt.output1)
                );

                println!(
                    "MD4Sum for {}: {}",
                    opt.output2.display(),
                    App::md4sum(&opt.output2)
                );

                if md4sum1 == md4sum2 {
                    println!("MD4Sum is identical.");
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
