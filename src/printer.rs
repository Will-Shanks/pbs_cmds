use std::collections::{BTreeMap, HashSet};
use crate::Attribs;

pub fn print_status(status: &Vec<BTreeMap<String,String>>, attribs: &Attribs) {
    let attribs_map: HashSet<&String> = attribs.out.iter().collect();
    for stat in status {
        println!("{}", stat.get("name").unwrap());
        if attribs.no_attribs {
            continue
        }

        let mut stat_attribs: BTreeMap<String, String> = stat.iter().filter(|(k,_)| k != &"name")
            .map(|(k,v)| (k.to_string(), v.to_string())).collect();
        if attribs.out.len() != 0 {
            stat_attribs = stat_attribs.iter().filter(|(k,_)| attribs_map.contains(k))
                .map(|(k,v)| (k.to_owned(), v.to_owned())).collect();
        }

        if attribs.long {
            for (k,v) in stat_attribs {
                println!("\t{k}: {v}");
            }
        } else if attribs.json {
            todo!()
        } else {
            todo!()
        }
    }
}
