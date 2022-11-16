#![allow(unused_parens)]
#![allow(unused_imports)] //TODO: remove
#![allow(dead_code)] //TODO: remove

//CLi argument management
use clap::Parser;

use std::path::PathBuf;

//file management
use tempfile::tempfile;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Folder to convert [TODO: if file instead of folder]
	#[arg(short = None, long = None, value_name = "FOLDER/FILE", value_parser = parse_path, default_value = ".")]
	path: PathBuf,

	/// Start processing at FILE.txt alpha numerically (inclusive). [TODO]
	#[arg(short, long, value_name = "FILE")]
	from: Option<String>,

	/// Stop processing at FILE.txt alpha numerically (inclusive). [TODO]
	#[arg(short, long, value_name = "FILE")]
	to: Option<String>,

	/// Overwrite existing mp3 files instead of skipping [TODO]
	#[arg(short, long)]
	overwrite: bool,

	/// Recursively go into directories. [TODO]
	#[arg(short, long)]
	recurse: bool,

	/// Shutdown system after finishing. [TODO]
	#[arg(short, long)]
	shutdown: bool,

	/// Remove non alphanumeric characters and normal punctuation. [TODO]
	#[arg(short, long)]
	normalize: bool,

	/// Fix troublesome abbreviations. [TODO]
	#[arg(short, long)]
	abbreviations: bool,
}

// Parse the path into a Path, verify it exists
fn parse_path(p: &str) -> Result<PathBuf, String> {
	let path: PathBuf = PathBuf::from(p);
	if(path.exists()) {
		return(Ok(path) as Result<PathBuf, String>);
	}
	return(Err(format!("Supplied path ({}) does not exist", path.display())) as Result<PathBuf, String>);
}

const IN_EXT: &str = ".txt";
const OUT_EXT: &str = ".mp3";
const FIXED_EXT: &str = ".gtts-tmp"; //TODO

// lvl/LVL to level '=>"
// sd 'lvl' 'level' *.txt; sd 'LVL' 'level' *.txt; sd ' ' '' *.txt; sd '』' '' *.txt; sd '『' '' *.txt; sd '°' '' *.txt; sd ' Lv' ' level ' *.txt; sd ' lv' ' level ' *.txt

//able to press key to stop at next finished file

fn main() {
	let args: Args = Args::parse();

	println!("{}", args.path.display());//TODO: remove
	println!("is DIR:{}", args.path.is_dir());//TODO: remove
	println!("is File:{}", args.path.is_file());//TODO: remove

	
}
