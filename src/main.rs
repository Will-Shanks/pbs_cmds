use clap::{Parser, Subcommand};
use pbs::{Attrl, Resource, Server};


#[derive(Debug, Parser)]
#[command(name = "wpbs")]
#[command(about = "BusyBox style command for all things pbs", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    server: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Stat{
        resource: Resource,
        name: Option<String>, 
        #[arg(long, short)]
        attribs: Vec<String>,
    },
    #[command(arg_required_else_help = true)]
    Sub,
    #[command(arg_required_else_help = true)]
    Del,
}

fn main() {
    env_logger::init();
    let args = Cli::parse();
    let srv = if let Some(s) = args.server {
        Server::connect_to(&s)
    } else {
        Server::new()
    };
    let resp = match args.command {
        Commands::Stat{resource, name, attribs} => {
            let a: Vec<Attrl> = attribs.iter().map(|x| (x.as_str()).into()).collect();
            srv.stat(resource, name, a)
        },
        _ => todo!(),
    };
    for elem in resp {
        println!("{elem}");
    }
}
