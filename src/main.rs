use clap::Parser;

macro_rules! USAGE {
	() => {
		println!("Usage: gtts-batch [FOLDER] [OPTIONS]\n\
			\tConvert folder of text to voice using gtts-cli\n\
			Options:\n\
			\tFOLDER        Folder to convert. Defaults to \".\"\n\
			\t-c <count>    Stop processing on after the N files.\n\
			\t-w <time>     TIME in minutes between file conversions. Default to 5.\n\
			\t-h            Show this message and exit.");
	};
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

fn main() {
	let args = Args::parse();
}