use clap::{Parser, Subcommand};
use pbs::{Attribs, Server, Status, StatResp};
use std::collections::BTreeMap;


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
    Stat(Filter),
    Sub,
//    Del,
}

#[derive(Debug, Subcommand)]
enum StatVerb {
    Stat(Filter),
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

#[derive(Debug,Default, clap::Args)]
pub struct Filter {
    #[arg(help="name, or nameset ex: casper1[2-7]")]
    name: Option<String>,
    #[arg(short, long, help="filter attributes, ex: state=free, can use =, !=, <, <=, >, and >=")]
    attribs: Vec<String>,
    #[arg(short, long, conflicts_with="no_attribs", help="attributes to display, ex: state")]
    out: Vec<String>,
    #[arg(short, long, action, conflicts_with="out", help="don't print any attributes, only names")]
    no_attribs: bool,
    #[arg(short, long, action, conflicts_with="no_attribs", conflicts_with="json", help="print resources over multiple lines")]
    long: bool,
    #[arg(short, long, action, conflicts_with="long", help="output in json")]
    json: bool,
}

impl Filter {
    fn attribs(&self) -> Attribs {
        (&self.attribs).into()
    }
    fn out(&self) -> Attribs {
        (&self.out).into()
    }
    fn contains_name(&self, name: &str) -> bool {
        if self.name.is_none() { return true};
        for n in self.names() {
            if name == n {return true;}
        }
        false
    }
    fn names(&self) -> Vec<String> {
        if let Some(name) = &self.name {
            hostlist_parser::parse(name).unwrap()
        }else{
            vec!()
        }
    }
    fn check_filters(&self, status: &BTreeMap<String,String>) -> bool {
        todo!()
/*
        for attrib in self.attribs() {
            if let Some(actual) = status.get(&attrib.fullname()) {
                let valid = match attrib.op {
                    pbs::batch_op::EQ => attrib.value == actual,
                    pbs::batch_op::NE => attrib.value != actual,
                    pbs::batch_op::GE => todo!(),
                    pbs::batch_op::GT => todo!(),
                    pbs::batch_op::LE => todo!(),
                    pbs::batch_op::LT => todo!(),
                    _ => panic!("Invalid comparison type"),
                };
                if !valid {
                    return false
                }
            } else {
                return false
            }
        }
        true
*/
    }
}

fn handle_stat(data: &StatResp, attribs: &Filter) {
    for item in &data.resources {
        println!("{}",item.name());
        println!("{:?}",item.attribs())
    }
}
/*
    // filter out resources we don't care about
    let filtered_data = data.filter(|s| attribs.contains_name(s.name()))
        .map(|s| (s.name().to_string(), s.attribs())).map(|(n,mut s)| {s.insert("name".to_string(), n); s}).filter(|s| attribs.check_filters(s))
        .collect::<Vec<BTreeMap<String,String>>>();
    printer::print_status(&filtered_data, attribs);
*/

fn main() {
    env_logger::init();
    let args = Cli::parse();
    let srv = if let Some(s) = args.server {
        Server::connect_to(&s).unwrap()
    } else {
        Server::new()
    };
    match args.noun {
        Noun::Job(verb) => {
            match verb {
                Verb::Stat(attribs) => {
                    handle_stat(&mut srv.stat_job(attribs.attribs(), attribs.out()).unwrap(), &attribs);
                },
                Verb::Sub => {todo!()},
                //Verb::Del => {todo!()},
            }
        },
        Noun::Host(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full host stat if attribs.names().len() is short
                    let resp = srv.stat_host(&None, attribs.out());
                    handle_stat(&resp.unwrap(), &attribs);
                },
            }
        },
        Noun::Reservation(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    handle_stat(&mut srv.stat_reservation(&None, attribs.out()).unwrap(), &attribs);
                },
            }
        },
        Noun::Resource(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full if attribs.names().len() is short
                    handle_stat(&mut srv.stat_resource(&None, attribs.out()).unwrap(), &attribs);
                },
            }
        },
        Noun::Vnode(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    handle_stat(&mut srv.stat_vnode(&None, attribs.out()).unwrap(), &attribs);
                },
            }
        },
        Noun::Que(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    handle_stat(&mut srv.stat_que(&None, attribs.out()).unwrap(), &attribs);
                },
            }
        },
        Noun::Scheduler(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    handle_stat(&mut srv.stat_scheduler(&None, attribs.out()).unwrap(), &attribs);
                },
            }
        },
        Noun::Server(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    handle_stat(&mut srv.stat_scheduler(&None, attribs.out()).unwrap(), &attribs);
                },
            }
        },
    }
}
