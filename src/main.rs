use clap::{Parser, Subcommand};
use pbs::{Server};


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
    #[command(arg_required_else_help = true, subcommand)]
    Stat(Resource),
    #[command(arg_required_else_help = true)]
    Sub,
    #[command(arg_required_else_help = true)]
    Del,
}

#[derive(Debug, Subcommand)]
enum Resource {
    Job {
        #[arg(short, long)]
        attribs: Vec<String>,
        #[arg(short, long)]
        out: Vec<String>,
    },
    Host(Attribs),
    Reservation(Attribs),
    Resource(Attribs),
    #[command(name="vnode")]
    Vnode(Attribs),
    Que(Attribs),
    Scheduler(Attribs),
    Server(Attribs),
}

#[derive(Debug, clap::Args)]
struct Attribs {
    name: Option<String>,
    out: Vec<String>,
}

fn main() {
    env_logger::init();
    let args = Cli::parse();
    let srv = if let Some(s) = args.server {
        Server::connect_to(&s)
    } else {
        Server::new()
    };
    match args.command {
        Commands::Stat(r) => {
            match r {
                Resource::Job{attribs: a, out: o} => {
                    let a = a.iter().map(|x| x.as_str().into()).collect();
                    let o = o.iter().map(|x| x.as_str().into()).collect();
                    srv.stat_job(a, o).unwrap().for_each(|x| println!("{x}"))
                },
                Resource::Host(h) => srv.stat_host(h.name, h.out.iter().map(|x| x.as_str().into()).collect()).unwrap().for_each(|x| println!("{x}")),
                Resource::Reservation(h) => srv.stat_reservation(h.name, h.out.iter().map(|x| x.as_str().into()).collect()).unwrap().for_each(|x| println!("{x}")),
                Resource::Resource(h) => srv.stat_resource(h.name, h.out.iter().map(|x| x.as_str().into()).collect()).unwrap().for_each(|x| println!("{x}")),
                Resource::Vnode(h) => srv.stat_vnode(h.name, h.out.iter().map(|x| x.as_str().into()).collect()).unwrap().for_each(|x| println!("{x}")),
                Resource::Que(h) => srv.stat_que(h.name, h.out.iter().map(|x| x.as_str().into()).collect()).unwrap().for_each(|x| println!("{x}")),
                Resource::Scheduler(h) => srv.stat_scheduler(h.name, h.out.iter().map(|x| x.as_str().into()).collect()).unwrap().for_each(|x| println!("{x}")),
                Resource::Server(h) => srv.stat_server(h.name, h.out.iter().map(|x| x.as_str().into()).collect()).unwrap().for_each(|x| println!("{x}")),
            }
        },
        _ => todo!(),
    };
}
