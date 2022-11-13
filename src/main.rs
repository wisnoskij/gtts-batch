use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Folder to convert
	#[arg(short = None, long = None, value_name = "FOLDER",default_value_t = Path::new("."))]
	folder: Path,

	/// Start processing at FILE.txt alpha numerically (inclusive).
	#[arg(short, long, value_name = "FILE")]
	from: &'static str,

	/// Stop processing at FILE.txt alpha numerically (inclusive).
	#[arg(short, long, value_name = "FILE")]
	to: &'static str,
}

// single file mode
// lvl/LVL to level
// recursive down dirs

fn main() {
	let args: Args = Args::parse();
}
