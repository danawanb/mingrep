use std::env;

fn main() {
    let me = String::from("Mini Grep here");
    println!("{}", me);

    let args: Vec<String> = env::args().collect();

    let grep = grep::Config::new(args).unwrap();

    grep::run(grep).unwrap();

    println!("end");
}
