use clap::Parser;
use simple_logger::SimpleLogger;

#[derive(Parser, Debug, Clone)]
#[command(version, about = "A simple HTTP server.")]
struct CLIConfig {
    #[arg(short, long, default_value = "8080", help = "The port that server will be listening for requests on.")]
    port: u16,
    #[arg(short, long, default_value = ".", help = "The directory that will be served.")]
    directory: String,
    #[arg(short, long, default_value = "index.html", help = "The file that will be read from requested path when user requests url ending with '/'.")]
    index: String
}


fn main() {
    SimpleLogger::new().init();

    let cli_config = CLIConfig::parse();
}