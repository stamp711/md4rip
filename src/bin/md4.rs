use md4rip::App;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
#[structopt(name = "md4", about = "MD4 calculator")]
pub struct Opt {
    /// The input file to calculate MD4
    #[structopt(name = "PATH", parse(try_from_os_str = "parse_existing_file"))]
    path: PathBuf,
}

fn parse_existing_file(s: &OsStr) -> Result<PathBuf, OsString> {
    let p = PathBuf::from(s);
    if p.exists() {
        Ok(p)
    } else {
        Err(OsString::from("input file is not valid"))
    }
}

fn main() {
    let md4sum = App::md4sum(&Opt::from_args().path);
    println!("{}", md4sum);
}
