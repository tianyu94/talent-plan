#[macro_use]
extern crate clap;
#[macro_use]
extern crate dotenv_codegen;

use clap::App;
use std::fmt::Formatter;
// fn main() {
//     let yaml = load_yaml!("cli.yml");
//     let matches = App::from_yaml(yaml).get_matches();
// }

// fn main() {
//     println!("{}", dotenv!("PORT"));
// }

// fn main() {
//     let key = "HOME";
//     match std::env::var_os(key) {
//         Some(val) => println!("{}: {:?}", key, val),
//         None => println!("{} is not defined in the environment", key),
//     }
// }

enum MyErr {
    R1(String),
    R2(String, u32),
}
impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyErr::R1(ref s) => write!(f, "`{}` is the error", s),
            MyErr::R2(ref s, ref n) => write!(f, "`{}` and `{}` are error", s, n),
        }
    }
}
fn foo() -> Result<(), MyErr> {
    Err(MyErr::R2(String::from("foo"), 2))
}
fn main() {
    println!("{}", MyErr::R1(String::from("Test R1")));
    println!("{}", MyErr::R2(String::from("Test R2"), 12));
    let result = foo();
    if let Some(e) = result.err() {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}
