use clap::{Parser, Subcommand};

mod build;
mod send;
mod cli_lib;

const SEND_ABOUT: &str = "Send requests to a remote server";
const BUILD_ABOUT: &str = "Insert specified JSON data into a predefined Request template `body`";

#[derive(Subcommand, Debug, Clone)]
enum NetCommands {
  #[command(about=SEND_ABOUT, long_about=None)]
  Send {
    #[arg(long, short='f', num_args=1..)]
    files_path: Vec<String>,

    #[arg(short, long, default_value_t=false)]
    write: bool,
  },

  #[command(about=BUILD_ABOUT, long_about=None)]
  Build {
    #[arg(long, short='f', num_args=1..)]
    files_path: Vec<String>,

    #[arg(long, short='o')]
    output_path: Option<String>,
  },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Arguments {
  #[clap(subcommand)]
  net: NetCommands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let args = Arguments::parse();

  match args.net {
    NetCommands::Build { files_path, output_path } => 
      build::handle_build(&files_path, &output_path),
      
    NetCommands::Send { files_path, write } => 
      send::handle_send(&files_path, write).await,
  }

  return Ok(())
}
