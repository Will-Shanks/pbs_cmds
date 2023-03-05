use clap::{Parser, Subcommand};
use pbs::{Attribs, Server, Status, StatResp};
use serde_json;


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
enum JobVerb {
    Stat(Filter),
    Sub(Submit),
    Del(DelAttribs),
}

#[derive(Debug, Subcommand)]
enum ResvVerb {
    Stat(Filter),
    Sub(Submit),
    Del(DelAttribs),
    Mod(ModAttribs),
}

#[derive(Debug, Subcommand)]
enum StatVerb {
    Stat(Filter),
}

#[derive(Debug, Subcommand)]
enum Noun {
    #[command(subcommand)]
    Job(JobVerb),
    #[command(subcommand)]
    Host(StatVerb),
    #[command(subcommand, name="resv")]
    Reservation(ResvVerb),
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
pub struct ModAttribs {
    #[arg(help="name")]
    name: String,
    #[arg(help="attributes")]
    attribs: Vec<String>,
}

#[derive(Debug,Default, clap::Args)]
pub struct DelAttribs {
    #[arg(help="name")]
    name: String,
}

#[derive(Debug,Default, clap::Args)]
pub struct Submit {
    #[arg(help="attributes")]
    attribs: Vec<String>,
    #[arg(short, long, help="job script")]
    script: String,
    #[arg(short, long, help="job que")]
    que: String,
    #[arg(short, long, action, help="don't print any attributes, only names")]
    no_attribs: bool,
    #[arg(short, long, action, conflicts_with="no_attribs", conflicts_with="json", help="print resources over multiple lines")]
    long: bool,
    #[arg(short, long, action, conflicts_with="long", help="output in json")]
    json: bool,
}

#[derive(Debug,Default, clap::Args)]
pub struct Filter {
    #[arg(help="name, or nameset ex: casper1[2-7]")]
    name: Option<String>,
    #[arg(short, long, help="filter attributes, ex: state=free, can use =, !=, <, <=, >, and >=")]
    attribs: Vec<String>,
    #[arg(short, long, conflicts_with="no_attribs", help="attributes to display, ex: state")]
    out: Option<Vec<String>>,
    #[arg(short, long, action, conflicts_with="out", help="don't print any attributes, only names")]
    no_attribs: bool,
    #[arg(short, long, action, conflicts_with="no_attribs", conflicts_with="json", help="print resources over multiple lines")]
    long: bool,
    #[arg(short, long, action, conflicts_with="long", help="output in json")]
    json: bool,
}

enum Printfmt {
    Json,
    Long,
    Short,
}

impl Submit {
    fn attribs(&self) -> Attribs {
        (&self.attribs).into()
    }
}
impl ModAttribs {
    fn attribs(&self) -> Attribs {
        (&self.attribs).into()
    }
}

impl Filter {
    fn attribs(&self) -> Attribs {
        (&self.attribs).into()
    }
    fn out(&self) -> Option<Attribs> {
        match &self.out {
            None => None,
            Some(x) => Some(x.into()),
        }
    }
    fn names(&self) -> Vec<String> {
        if let Some(name) = &self.name {
            hostlist_parser::parse(name).unwrap()
        }else{
            vec!()
        }
    }
    fn check(&self, status: &Status) -> bool {
        if self.name.is_some() && !self.names().contains(&status.name()) {return false}
        if !status.attribs().check_filter(&self.attribs()) {return false}
        true
    }
    fn printfmt(&self) -> Printfmt {
        if self.json { return Printfmt::Json; }
        if self.long { return Printfmt::Long }
        Printfmt::Long
        //Printfmt::Short
    }
}

fn handle_stat(data: &StatResp, attribs: &Filter) {
    let data = data.resources.iter().filter(|r| attribs.check(r));
    match attribs.printfmt() {
        Printfmt::Long => {
            data.for_each( |item| {
                println!("{}",item.name());
                println!("{}",item.attribs());
            });
        },
        Printfmt::Short => todo!(),
        Printfmt::Json  => {
            let out: Vec<_> = data.map(|item|{
                let mut attribs = item.attribs().json();
                attribs.as_object_mut().unwrap().insert("name".to_string(), serde_json::Value::String(item.name()));
                attribs
            }).collect();
            println!("{}", serde_json::to_string(&out).unwrap());
        },
    }
}

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
                JobVerb::Stat(attribs) => {
                    handle_stat(&srv.stat_job(attribs.attribs(), attribs.out()).unwrap(), &attribs);
                },
                JobVerb::Sub(attribs) => {
                    let resp = &srv.submit_job(attribs.attribs(), &attribs.script, &attribs.que);
                    println!("{resp:?}");
                },
                JobVerb::Del(attribs)  => {
                    if let Err(e) = &srv.del_job(&attribs.name) {
                        println!("Error deleting job: {e}");
                    } else {
                        println!("Job deleted");
                    }
                },
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
                ResvVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    handle_stat(&srv.stat_reservation(&None, attribs.out()).unwrap(), &attribs);
                },
                ResvVerb::Sub(attribs) => {
                    match &srv.submit_resv(attribs.attribs()) {
                        Err(e) => println!("Error submitting reservation: {e}"),
                        Ok(id) => println!("Reservation submitted id: {id}"),
                    }
                },
                ResvVerb::Mod(attribs) => {
                    match &srv.mod_resv(&attribs.name, attribs.attribs()) {
                        Err(e) => println!("Error submitting reservation: {e}"),
                        Ok(id) => println!("Reservation submitted id: {id}"),
                    }
                },
                ResvVerb::Del(attribs) => {
                    if let Err(e) = &srv.del_resv(&attribs.name) {
                        println!("Error deleting reservation: {e}");
                    } else {
                        println!("Reservation deleted");
                    }
                },
            }
        },
        Noun::Resource(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full if attribs.names().len() is short
                    handle_stat(&srv.stat_resource(&None, attribs.out()).unwrap(), &attribs);
                },
            }
        },
        Noun::Vnode(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    handle_stat(&srv.stat_vnode(&None, attribs.out()).unwrap(), &attribs);
                },
            }
        },
        Noun::Que(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    handle_stat(&srv.stat_que(&None, attribs.out()).unwrap(), &attribs);
                },
            }
        },
        Noun::Scheduler(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    handle_stat(&srv.stat_scheduler(&None, attribs.out()).unwrap(), &attribs);
                },
            }
        },
        Noun::Server(verb) => {
            match verb {
                StatVerb::Stat(attribs) => {
                    //TODO don't do a full stat if attribs.names().len() is short
                    handle_stat(&srv.stat_scheduler(&None, attribs.out()).unwrap(), &attribs);
                },
            }
        },
    }
}
