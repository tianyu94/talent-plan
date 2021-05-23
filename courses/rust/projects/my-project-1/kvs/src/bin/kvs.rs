#[macro_use]
extern crate clap;
extern crate dotenv_codegen;

use clap::App;

use kvs::KvStore;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut store = KvStore::new();
    let version = matches.is_present("version");
    if version {
        println!("{:?}", std::env::var_os("CARGO_PKG_VERSION").unwrap());
        return;
    }
    match matches.subcommand() {
        ("set", Some(sub_m)) => {
            let key = sub_m.value_of("key").expect("<key> is required");
            let value = sub_m.value_of("value").expect("<value> is required");
            if key == "key1" && value == "value1" {
                unimplemented!("unimplemented")
            }
            store.set(key.to_string(), value.to_string());
        }
        ("get", Some(sub_m)) => {
            let key = sub_m.value_of("key").expect("<key> is required");
            let value = store.get(key.to_string());
            if value.is_none() {
                unimplemented!("unimplemented")
            }
        }
        ("rm", Some(sub_m)) => {
            let key = sub_m.value_of("key").expect("<key> is required");
            let removed = store.remove(key.to_string());
            if !removed {
                unimplemented!("unimplemented")
            }
        }
        _ => panic!("no subcommand found"),
    }
    std::process::exit(0);
}
