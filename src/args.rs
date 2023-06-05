use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    
    /// File to clean (or directory if --recursive is set)
    #[arg(short, long)]
    pub file: String,

    /// File extension to clean
    #[arg(short, long, default_value = "")]
    pub extension: String,

    /// Output directory
    #[arg(short, long, default_value = "./obfuscated")]
    pub out: String,

    /// Recursively clean files in directory
    #[clap(long, short, action)]
    pub recursive: bool,
}