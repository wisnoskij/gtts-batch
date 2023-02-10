#![allow(unused_parens)]
#![allow(unused_imports)] //TODO: remove
#![allow(dead_code)] //TODO: remove
#![allow(unused_variables)] //TODO: remove
#![allow(unused_assignments)] //TODO: remove
#![allow(unused_mut)] //TODO: remove

//CLi argument management
use clap::Parser;

use std::ffi::OsStr;
use std::fs::{self, ReadDir, rename, File};
use std::io::{self, Seek, Read, BufReader, BufRead, Write, ErrorKind};
use std::iter::Filter;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::{thread, time::Duration};

const IN_EXT: &str = "txt";
const OUT_EXT: &str = "mp3";
const OUT_TMP: &str = "gtts_mp3";
const FIXED_EXT: &str = "gtts_txt";

/// Wrapper over gtts to handle large conversions. Batch converting many files,
/// and spitting up large files into multiple smaller files.


// Works with Clap to handle and store all the command lines arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args{
	/// Folder to convert
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

	/// Recursively go into directories.
	#[arg(short, long)]
	recurse: bool,

	/// Shutdown system after finishing. [TODO]
	#[arg(short = None, long)]
	shutdown: bool,

	/// Remove non alphanumeric characters and normal punctuation. [TODO]
	///
	/// Done after splitting, so these unusual characters can be used for splitting purposes.
	#[arg(short, long)]
	normalize: bool,

	/// Fix troublesome abbreviations. [TODO]
	///
	/// "LV" is considered some currency, so I fix that as well as other level abbreviations.
	/// "MP" Is read as Mega Pixel.
	/// Done after splitting, so these technically these fixes can make a split go over the max length.
	/// [TODO]: some method to pick which to apply.
	#[arg(short, long)]
	abbreviations: bool,

	/// The <MINUTES> to wait in minutes.
	#[arg(short, long, value_name = "MINUTES", default_value_t = 5)]
	wait: u64,

	/// The <MILI> to wait in milliseconds (1/1000th of a second).
	///
	/// wait<time in minutes> is ignored if this argument is present.
	#[arg(short = None, long, value_name = "MILI")]
	waitms: Option<u64>,

	/// Split file(s) at every occurrence of <STRING>.
	///
	/// <STRING> begins the split.
	/// Designed as the main user facing split mechanic. Designed around splitting by chapter.
	/// If present this is the first file modification run.
	#[arg(short, long, value_name = "STRING")]
	split: Option<String>,

	/// The Max length in bytes a single file can be before it gets split.
	///
	/// Use max length to figure out number of splits a file needs,
	/// then split at average. This can cause trouble if the file is so evenly split that
	/// their is no enough space to look for goods locations to split at.
	#[arg(short, long, value_name = "MAX BYTES", default_value_t = 40000)]
	max: usize,

	/// The Max variance from MAX LENGTH in bytes to search for splits.
	///
	/// Start searching at for a place to split a file at MAX, read backwards until MAX VARIANCE
	#[arg(short = None, long, value_name = "MAX VARIANCE", default_value_t = 4000)]
	max_var: usize,

	/// The string(s) to split at if file is over max length.
	///
	/// Tries to split at first string, if this fails moves to second and so on. If all fail
	/// just splits at the exact character.
	/// Split happens after STRING.
	#[arg(short = None, long, value_name = "STRING", default_values_t = 
		vec![String::from("\n\n"), String::from("\n"), String::from(".")])]
	split_strs: Vec<String>,

	/// Runs in testing mode, does everything except for calling google translate services and waiting between files.
	/// [TODO]: prevent shutoff from running if in testing mode.
	#[arg(short = None, long)]
	test: bool
}

// Parse the &str into a PathBuf, verify it exists.
fn parse_path(p: &str) -> Result<PathBuf, String> {
	let path: PathBuf = PathBuf::from(p);
	if(path.exists()){
		return(Ok(path) as Result<PathBuf, String>);
	}
	return(Err(format!("Supplied path ({}) does not exist", path.display())) as Result<PathBuf, String>);
}

// lvl/LVL to level '=>"
// sd 'lvl' 'level' *.txt; sd 'LVL' 'level' *.txt; sd ' ' '' *.txt; sd '』' '' *.txt; sd '『' '' *.txt; sd '°' '' *.txt;
// sd ' Lv' ' level ' *.txt; sd ' lv' ' level ' *.txt

//able to press key to stop at next finished file

// Holds list of files
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
			// Only care about mp3s if I am overwriting
			Some(OUT_EXT) if self.files_mp3.capacity() > 0 => self.files_mp3.push(file),
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

	args = calc_wait(args);

	batch(args);
}

// calc wait time, in ms, store it args.wait.
fn calc_wait(mut args: Args) -> Args{
	if(args.waitms.is_some()){
		// if milisecond wait exists, use that value
		args.wait = args.waitms.unwrap();
	}else{
		// else, use the minute value (calc the ms value)
		args.wait *= 60000;
	}
	return(args);
}

fn batch(mut args: Args) -> Args{
	let mut files: Files; 

	(files, args) = order_files(args); // Read in Files

	// Warn/inform user the general scope of the proposed action.
	// Note that we are not recusing yet so dont really have any idea how many files are being converted.
	// Could improve this and get an actual estimated time by reading in file size, but I dont think that is needed.
	if(files.dirs.len() > 0){ 
		println!("Converting {} files in ({}) and recusing through {} directories.", files.files_txt.len()
			, args.path.to_str().expect("The path should be readable"), files.dirs.len());
	}else{
		println!("Converting {} files in ({}).", files.files_txt.len(), args.path.to_str()
			.expect("The path should be readable"));
	}

	(files, args) = iter_files(files, args); // Process Files

	for dir in files.dirs{
		args.path = dir;
		args = batch(args);
	}
	return(args);
}

// Sort files into struct
fn order_files(args: Args) -> (Files, Args){
	let mut files: Files = Files::new(args.overwrite, args.recurse);
	let mut tmp_path: PathBuf;

	if(args.path.is_dir()){
		files = read_dir(files, &args);
	} else if(args.path.is_file()){
		files.push(args.path.to_path_buf());
	}

	files.sort();
	files.shrink();
	return(files, args);
}

fn read_dir(mut files: Files, args: &Args) -> Files{
	for path in args.path.read_dir().expect("The path `FOLDER` is a directory and should be readable."){
		files.push(path.expect(
			"The directory `FOLDER` is readable, so it should not be erroring while iterating over it").path());
	}
	return(files);
}

fn iter_files(files: Files, args: Args) -> (Files, Args){
	let mut file_mp3: PathBuf;
	let mut file_tmp: PathBuf;
	let mut split_count: u16; // Used to track the number of splits a file gets to iterate the output name
	let mut not_first: bool = false; // Flag used to run thread sleep code between runs
	let mut reader: BufReader<File>;
	let mut buf: Vec<u8> = Vec::with_capacity(40000); //temp buf
	let mut buf_splits: Option<Vec<usize>>; // Vector holding the splits of buf. Each element is the length next section.
	let mut buf_index: usize;
	let mut changed_flag: bool;

	for file_txt in &files.files_txt {
		split_count = 0;
		file_mp3 = file_txt.to_path_buf();
		file_mp3.set_extension(OUT_EXT);

		// Skip files with already existing mp3s, unless we are overwriting
		if(files.contains(&file_mp3)){
			println!("Skipping {}. An mp3 already exists.", file_mp3.to_str()
				.expect("The file's path should be readable"));
			continue;
		}

		// Wait between calls to google translate services to not overload their servers. Skip if in testing mode.
		if(!args.test && not_first){
			thread::sleep(Duration::from_millis(args.wait.clone()));
		} not_first = true;

		changed_flag = false;
		reader = BufReader::new(File::open(file_txt).expect("The file should exist and be readable."));

		while(has_data_left(&mut reader).expect("The buffer should be readable/mutable.")) {

			// Check if I need to split at --split, read to split (into buf), else read to end.
			if let Some(ref split_str) = args.split { // Split files at --split <STRING>
				(buf, reader) = split_at_str(split_str.as_bytes(), buf, reader);
				if(!changed_flag && has_data_left(&mut reader)
					.expect("The buffer should be readable/mutable.")){
					changed_flag = true
				}
			} else {
				reader.read_to_end(&mut buf).expect("The buffer should be readable/mutable.");
			}

			// Check if buf is too long (> --max). Return Iterator with the ending index of each section.
			buf_index = 0;
			for buf_splits in split_at_max(&buf, args.split_str.clone(), args.max, args.max_var){

				//if(args.normalize)TODO
				//}if(args.abbreviations){}TODO


				// If the input file has been split or modified in anyway
				// store the changed/split input into a tmp file
				if(changed_flag || buf_splits != buf.len()) {
					split_count += 1;
					// TODO: figure out how to just return a cloned copy at the forloop iter
					file_tmp = file_txt.clone();
					file_tmp.set_extension(format!("{:03}.{}", split_count, FIXED_EXT));
					File::create(&file_tmp).expect("Create a tmp file")
					.write_all(&buf[buf_index..buf_splits]).expect("Write buf to tmp file");
					buf_index = buf_splits+1; // Set index to the start of next section.

					gtts(&file_tmp, &file_mp3, args.test);
					fs::remove_file(file_tmp).expect("Delete tmp file after use.");
				}else{
					gtts(&file_txt, &file_mp3, args.test);
				}
			}
			buf.clear(); // Clear buf after writing
		}
	}
	return(files, args);
}


// Calc vector containing the ending index of each section to split buf at
// to fit within Max, ideally being split at one of the split strings. 
fn split_at_max(buf: &Vec<u8>, split_strs: Vec<String>, max: usize, max_variance: usize) -> Vec<usize> {
	if((max == 0) || (buf.len() <= max)) {
		return(vec![buf.len() - 1]);
	}
	let mut splits: Vec<usize> = Vec::new();
	let index: usize = 0;
	let index_of: Vec<usize> = Vec::with_capacity(split_strs.len());
	let split_u8vec: Vec<Vec<u8>> = split_strs.into_iter().map(|s| s.into_bytes()).collect();


	// The average split length to evenly split buf with all pieces being less than max
	// Equates to CEIL(length / CEIL(length/max))
	let mut split_size: usize = (buf.len() + max - 1) / max;
	split_size = (buf.len() + split_size - 1) / split_size;

	while(index < buf.len()) {
		if(index + max >= buf.len()){ // If the rest of the buf is less than max size
			splits.push(buf.len() - 1);
			return(splits);
		}

		index = find_split(buf, split_u8vec, index, max, max_variance);
		splits.push(index);
	}
	return(splits);
}

// Loop from the max value of the next split, backwards, for a length of max_variance
fn find_split(buf: &Vec<u8>, split_u8vec:Vec<Vec<u8>>, index:usize, max:usize, max_variance:usize) -> usize {
	for str_check in split_u8vec {
		for next_split_index in ((index + max - max_variance)..(index + max)).rev() {
			if(buf[next_split_index..].starts_with(&str_check))	{
				return(next_split_index + str_check.len());
			}
		}
	}
	return(index + max);
}

fn split_at_str(split_str: &[u8], mut buf: Vec<u8>, mut reader: BufReader<File>) -> (Vec<u8>, BufReader<File>) {
	reader.read_until(split_str[split_str.len() - 1], &mut buf).expect("The file should be readable");

	// If the reader starts with the split, look for next split
	if(buf.len() == split_str.len()){
		reader.read_until(split_str[split_str.len() - 1], &mut buf).expect("The file should be readable");
	}

	if(!has_data_left(&mut reader).expect("The buffer should be readable/mutable.")){
		return(buf, reader);
	}
	if(buf.ends_with(split_str)){ // If found and read the split_str, remove from buf, add back to reader.
		// Used saturating_sub to handle the case where there aren't N elements in the vector
		buf.truncate(buf.len().saturating_sub(split_str.len()));
		reader.seek_relative(i64::try_from(split_str.len()) // seek negative length of split_str
			.expect("split_str better be WAY smaller than an i64 or something really weird is going on.") * -1)
		.expect("Should be able to unseek the search string");
		return(buf, reader);
	}
	return(split_at_str(split_str, buf, reader));
}

// Replacement for unstable BufReader.has_data_left() TODO: Replace with official fn when stable
fn has_data_left(reader: &mut BufReader<File>) -> io::Result<bool> {
	// Read single byte (passing single element array to "hold" that byte)
	match reader.read_exact(&mut [0]) {
		Ok(_) => {
			//unread single byte, then return
			reader.seek_relative(-1).expect("Should be able to unseek the single byte I just read");
			return(Ok(true));
		},
		Err(e) if e.kind() == ErrorKind::UnexpectedEof => return(Ok(false)),
		Err(e) => return(Err(e)),
	};
}

// Handles interfacing with gtts-cli
fn gtts(in_file: &PathBuf, out_file: &PathBuf, test: bool){
	let mut out_file_tmp: PathBuf = in_file.clone();
	out_file_tmp.set_extension(OUT_TMP);

	if(!check_not_exist(&out_file_tmp) || !check_not_exist(&out_file)){ // in case it is a directory
		println!("Skipping {}", in_file.to_str().expect("The file's path should be readable"));
		return;
	}

	let mut command: Command = Command::new("gtts-cli");
	command.args(["--lang", "en", "--file",
		in_file.to_str().expect("The file's path should be readable"),
		"--output",
		//gtts-cli always overwrites by default on my system
		out_file_tmp.to_str().expect("The file's path should be readable"),
		]);

	println!("\nConverting: {}", in_file.to_str().expect("The file's path should be readable"));

	if(test){ return; }

	// Make gtts sys call, print output, then IF success rename
	if(print_output(command.output().expect("gtts-batch should be able to make system calls")).status.success()){
		rename_tmp_fin(out_file_tmp, out_file);
	}
}

// Print output of gtts_cli sys call
fn print_output(gtts_output: Output) -> Output{
	io::stdout().write_all(&gtts_output.stdout).expect("gtts-batch should be able to write to stdout");
	io::stderr().write_all(&gtts_output.stderr).expect("gtts-batch should be able to write to stderr");
	io::stderr().flush().expect("gtts-batch should be able to flush stderr");
	println!("{}", gtts_output.status);

	return(gtts_output);
}

// Rename temp file to final output file (gtts_tmp to .mp3)
fn rename_tmp_fin(out_file_tmp: PathBuf, out_file: &PathBuf) -> (PathBuf, &PathBuf){
	let move_result: io::Result<()> = std::fs::rename(out_file_tmp.to_str().expect("The file's path should be readable")
		, out_file.to_str().expect("The file's path should be readable"));
	if move_result.is_ok(){
		println!("Conversion Succeeded.");
	}else{
		println!("Conversion Failed.");
	}
	return (out_file_tmp, out_file);
}

// Check if path is a directory
fn check_not_exist(file: &PathBuf)->bool{
	if file.is_dir(){
		println!("{} is a directory, and it should not be. Please delete, rename, or move this directory"
			, file.to_str().expect("The file's path should be readable"));
		return(false);
	}
	return(true);
}