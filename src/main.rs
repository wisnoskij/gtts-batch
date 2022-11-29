#![allow(unused_parens)]
#![allow(unused_imports)] //TODO: remove
#![allow(dead_code)] //TODO: remove
#![allow(unused_variables)] //TODO: remove
#![allow(unused_assignments)] //TODO: remove
#![allow(unused_mut)] //TODO: remove

//CLi argument management
use clap::Parser;

use std::ffi::OsStr;
use std::fs::{ReadDir, rename};
use std::path::PathBuf;
use std::process::{Command, Output};
use std::io::{self, Write};
//use std::{thread, time};

const IN_EXT: &str = "txt";
const OUT_EXT: &str = "mp3";
const OUT_TMP: &str = "gtts_mp3";
const FIXED_EXT: &str = "gtts_txt"; //TODO: Use


/// Wrapper over gtts to handle large conversions. Batch converting many files,
/// and spitting up large files into multiple smaller files.


/// Works with Clap to handle and store all the command lines arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
	/// Folder to convert [TODO: if file instead of folder]
	#[arg(short = None, long = None, value_name = "FOLDER/FILE", value_parser = parse_path, default_value = ".")]
	path: PathBuf,

	/// Start processing at FILE.txt alpha numerically (inclusive). [TODO]
	#[arg(short, long, value_name = "FILE")]
	from: Option<String>,

	/// Stop processing at FILE.txt alpha numerically (inclusive). [TODO]
	#[arg(short, long, value_name = "FILE")]
	to: Option<String>,

	/// Overwrite existing mp3 files instead of skipping
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
	///
	/// "LV" is considered some currency, so I fix that as well as other level abreviations.
	#[arg(short, long)]
	abbreviations: bool,

	/// The <MINUTES> to wait in minutes.
	#[arg(short, long, value_name = "MINUTES", default_value_t = 5)]
	wait: u16,

	/// The <MILI> to wait in miliseconds (1/1000th of a second).
	///
	/// wait<time in munutes> is ignored if this argument is present.
	#[arg(short = None, long, value_name = "MILI")]
	waitms: Option<u64>,

	/// Split file(s) at every occurance of <STRING>. [TODO]
	///
	/// <STRING> begins the split.
	/// This happens first, before checking for max length
	#[arg(short = None, long, value_name = "STRING")]
	split: Option<String>,

	/// The max length in bytes a single file can be before it gets split. [TODO]: Figure out if I am splitting by character or byte.
	#[arg(short, long, value_name = "BYTES", default_value_t = 40000)]
	max: u32,

	/// The string(s) to split at.
	///
	/// Tries to split at first string, if this fails moves to second and so on. If all fail, just splits at the exact character.
	/// Split happens after STRING.
	#[arg(short = None, long, value_name = "STRING", default_values_t = vec![String::from("\n\n"), String::from("\n"), String::from(".")])]
	splitstr: Vec<String>
}

// Parse the &str into a PathBuf, verify it exists.
fn parse_path(p: &str) -> Result<PathBuf, String>{
	let path: PathBuf = PathBuf::from(p);
	if(path.exists()){
		return(Ok(path) as Result<PathBuf, String>);
	}
	return(Err(format!("Supplied path ({}) does not exist", path.display())) as Result<PathBuf, String>);
}

//TODO: Check how many files will be overwritten and print to user.

// lvl/LVL to level '=>"
// sd 'lvl' 'level' *.txt; sd 'LVL' 'level' *.txt; sd ' ' '' *.txt; sd '』' '' *.txt; sd '『' '' *.txt; sd '°' '' *.txt; sd ' Lv' ' level ' *.txt; sd ' lv' ' level ' *.txt

//able to press key to stop at next finished file

// Holds list of files
#[derive(Debug)]
struct Files{
	files_txt: Vec<PathBuf>,
	files_mp3: Vec<PathBuf>,
	dirs: Vec<PathBuf>,
}
impl Files{
	fn new(overwrite: bool, recurse: bool) -> Files{
		Files{
			files_txt : Vec::with_capacity(101),
			// If overwriting files, I dont need to worry about existing mp3s
			files_mp3 : if(overwrite){ Vec::with_capacity(0) }else{ Vec::with_capacity(101) },
			// If not recursing, I dont need to worry about directories
			dirs : if(recurse){ Vec::with_capacity(11) }else{ Vec::with_capacity(0) },
		}
	}
	fn sort(&mut self){
		self.files_txt.sort();
		self.files_mp3.sort();
		self.dirs.sort();
	}
	fn shrink(&mut self){
		self.files_txt.shrink_to_fit();
		self.files_mp3.shrink_to_fit();
		self.dirs.shrink_to_fit();
	}
	fn push_file(&mut self, file: PathBuf){
		match(file.extension().and_then(OsStr::to_str)){
			None =>{},
			Some(IN_EXT) => self.files_txt.push(file),
			Some(OUT_EXT) if self.files_mp3.capacity() > 0 => self.files_mp3.push(file), // Only care about mp3s if I am overwriting
			Some(&_) =>{},
		}
	}
	fn push_dir(&mut self, file: PathBuf){
		if(self.dirs.capacity() > 0 ){
			self.dirs.push(file);
		}
	}
	fn push(&mut self, file: PathBuf){
		if(file.is_dir()){
			self.push_dir(file);
		}else if(file.is_file()){
			self.push_file(file);
		}
	}
	// Checks if their was a already converted mp3 by the same name.
	// Setting the overwrite argument garentees the array will be empty, so will alwasys return true.
	fn contains(&self, file: &PathBuf)->bool{
		return(self.files_mp3.binary_search(file).is_ok());
	}
}

fn main(){
	let mut args: Args = Args::parse();
	
	batch(&mut args);
}

fn batch(args: &mut Args){
	let mut files: Files;

println!("START: {}\n\n", args.path.to_str().expect("dd"));
	files = order_files(&args); // Read in Files
	iter_files(&files); // Process Files

	for dir in files.dirs{
		args.path = dir;
		batch(args);
	}
}

// Sort files into struct
fn order_files(args: &Args) -> Files{
	let mut files: Files = Files::new(args.overwrite, args.recurse);
	let mut tmp_path: PathBuf;

	if(args.path.is_dir()){
		read_dir(&mut files, &args);
	} else if(args.path.is_file()){
		files.push(args.path.to_path_buf());
	}
	
	files.sort();
	files.shrink();
	return(files);
}

fn read_dir(files: &mut Files,args: &Args){
	for path in args.path.read_dir().expect("The path `FOLDER` is a directory and should be readable."){
		files.push(path.expect("The directory `FOLDER` is readable, so it should not be erroring while iterating over it").path());
	}
}

fn iter_files(files: &Files){
	let mut file_mp3: PathBuf;
	for file_txt in &files.files_txt{
		file_mp3 = file_txt.to_path_buf();
		file_mp3.set_extension(OUT_EXT);
		if(files.contains(&file_mp3)){ continue; }
		gtts(file_txt.to_path_buf(), file_mp3);
	}
}

// Handles interfacing with gtts-cli
fn gtts(in_file: PathBuf, out_file: PathBuf){
	let mut out_file_tmp: PathBuf = in_file.clone();
	out_file_tmp.set_extension(OUT_TMP);

	if !check_not_exist(&out_file_tmp) ||
	!check_not_exist(&out_file){ //incase it is a directory
		println!("Skipping {}", in_file.to_str().expect("The file's path should be readable"));
		return;
	}

	let mut command: Command = Command::new("gtts-cli");
	command.args(["--lang", "en", "--file"]);
	command.args([
		in_file.to_str().expect("The file's path should be readable"),
		"--output",
		out_file_tmp.to_str().expect("The file's path should be readable"), //gtts-cli alwasys overwrites by default on my system
		]);
	println!("gtts-cli --lang en --file {} --output {}", in_file.to_str().expect("The file's path should be readable"), out_file_tmp.to_str().expect("The file's path should be readable"));
	
	//if(true){ return; }

	let gtts_output: Output = command.output().expect("gtts-batch should be able to make system calls");
	io::stdout().write_all(&gtts_output.stdout).expect("gtts-batch should be able to write to stdout");
	io::stderr().write_all(&gtts_output.stderr).expect("gtts-batch should be able to write to stderr");
	io::stderr().flush().expect("gtts-batch should be able to flush stderr");
	println!("Status: {}", gtts_output.status);

	let move_result: io::Result<()> = std::fs::rename(out_file_tmp.to_str().expect("The file's path should be readable"), out_file.to_str().expect("The file's path should be readable"));
	if move_result.is_ok(){
		println!("{}'s conversion to {} finsihed.", in_file.to_str().expect("The file's path should be readable"), out_file.to_str().expect("The file's path should be readable"));
	}else{
		println!("{}'s conversion to {} failed.", in_file.to_str().expect("The file's path should be readable"), out_file.to_str().expect("The file's path should be readable"));
	}
}

// Check if path is a directory
fn check_not_exist(file: &PathBuf)->bool{
	if file.is_dir(){
		println!("{} is a directory, and it should not be. Please delete, rename, or move this directory", file.to_str().expect("The file's path should be readable"));
		return(false);
	}
	return(true);
}