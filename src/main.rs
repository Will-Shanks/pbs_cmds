use clap::{Parser, Subcommand};
use pbs::{Attrl, Server};


#[derive(Debug, Parser)]
#[command(name = "wpbs")]
#[command(about = "BusyBox style command for all things pbs", long_about = None)]
struct Cli {
    #[command(subcommand)]
    noun: Noun,
    #[arg(short, long)]
    server: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Verb {
    Stat(Attribs),
    Sub,
//    Del,
}

#[derive(Debug, Subcommand)]
enum StatVerb {
    Stat(Attribs),
}

#[derive(Debug, Subcommand)]
enum Noun {
    #[command(subcommand)]
    Job(Verb),
    #[command(subcommand)]
    Host(StatVerb),
    #[command(subcommand, name="resv")]
    Reservation(StatVerb),
    #[command(subcommand)]
    Resource(StatVerb),
    #[command(name="vnode", subcommand)]
    Vnode(StatVerb),
    #[command(subcommand)]
    Que(StatVerb),
    #[command(subcommand, name="sched")]
    Scheduler(StatVerb),
    #[command(subcommand, name="srv")]
    Server(StatVerb),
}

#[derive(Debug, clap::Args)]
struct Attribs {
    #[arg(help="name, or nameset ex: casper1[2-7]")]
    name: Option<String>,
    #[arg(short, long, help="filter attributes, ex: state=free, can use =, !=, <, <=, >, and >=")]
    attribs: Vec<String>,
    #[arg(short, long, help="attributes to display, ex: state")]
    out: Vec<String>,
}

impl Attribs {
    fn attribs(&self) -> Vec<Attrl> {
        self.attribs.iter().map(|x| x.as_str().into()).collect()
    }
    fn out(&self) -> Vec<Attrl> {
        self.out.iter().map(|x| x.as_str().into()).collect()
    }
    fn contains_name(&self, name: &str) -> bool {
        if self.name == None { return true};
        for n in self.names() {
            if name == n {return true;}
        }
        false
    }
    fn names(&self) -> Vec<String> {
        if let Some(name) = &self.name {
            hostlist_parser::parse(&name).unwrap()
        }else{
            vec!()
        }
    }
}

impl Default for Attribs {
    fn default() -> Self {
        Attribs{ name:None, attribs: vec!(), out: vec!()}
    }
}

fn main() {
    env_logger::init();
    let args = Cli::parse();
    let srv = if let Some(s) = args.server {
        Server::connect_to(&s)
    } else {
        Server::new()
    };
    match args.noun {
        Noun::Job(verb) => {
            match verb {
                Verb::Stat(attribs) => {
                    srv.stat_job(attribs.attribs(), attribs.out()).unwrap()
                        .filter(|s| attribs.contains_name(s.name()))
                        .for_each(|x| println!("{x}"));
                },
                Verb::Sub => {todo!()},
                //Verb::Del => {todo!()},
            }
        },
        Noun::Host(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full host stat if attribs.names().len() is short
                    srv.stat_host(&None, attribs.out()).unwrap()
                        .filter(|s| attribs.contains_name(s.name()))
                        .for_each(|x| println!("{x}"));
                },
            }
        },
        Noun::Reservation(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    srv.stat_reservation(&None, attribs.out()).unwrap()
                        .filter(|s| attribs.contains_name(s.name()))
                        .for_each(|x| println!("{x}"));
                },
            }
        },
        Noun::Resource(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full if attribs.names().len() is short
                    srv.stat_resource(&None, attribs.out()).unwrap()
                        .filter(|s| attribs.contains_name(s.name()))
                        .for_each(|x| println!("{x}"));
                },
            }
        },
        Noun::Vnode(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    srv.stat_vnode(&None, attribs.out()).unwrap()
                        .filter(|s| attribs.contains_name(s.name()))
                        .for_each(|x| println!("{x}"));
                },
            }
        },
        Noun::Que(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    srv.stat_que(&None, attribs.out()).unwrap()
                        .filter(|s| attribs.contains_name(s.name()))
                        .for_each(|x| println!("{x}"));
                },
            }
        },
        Noun::Scheduler(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    srv.stat_scheduler(&None, attribs.out()).unwrap()
                        .filter(|s| attribs.contains_name(s.name()))
                        .for_each(|x| println!("{x}"));
                },
            }
        },
        Noun::Server(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    srv.stat_scheduler(&None, attribs.out()).unwrap()
                        .filter(|s| attribs.contains_name(s.name()))
                        .for_each(|x| println!("{x}"));
                },
            }
        },
    }
}
