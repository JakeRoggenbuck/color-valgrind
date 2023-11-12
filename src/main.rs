use efcl::{bold, color, Color};
use regex::Regex;
use std::env;
use std::process::exit;
use std::process::Command;

fn main() {
    let mut args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        println!("Needs at least one arguments");
        exit(0);
    }

    // Remove first arg
    args.drain(0..1);

    let command: String = args
        .clone()
        .into_iter()
        .map(|x| x + " ")
        .collect::<String>();

    println!("Running on {}...", color!(Color::GREEN, &command));

    let out: std::process::Output = Command::new("valgrind")
        .args(args)
        .output()
        .expect("failed to execute process");

    let mut output: String = String::from_utf8_lossy(&out.stderr).to_string();

    let re = Regex::new(r"==\d*== ").unwrap();
    output = re.replace_all(output.as_str(), "").to_string();

    let n_bytes = Regex::new(r"(?P<a>[\d,\,]* bytes)").unwrap();
    let n_blocks = Regex::new(r"(?P<a>[\d,\,]* blocks)").unwrap();
    let n_allocs = Regex::new(r"(?P<a>[\d,\,]* allocs)").unwrap();
    let n_frees = Regex::new(r"(?P<a>[\d,\,]* frees)").unwrap();

    output = n_bytes
        .replace_all(output.as_str(), bold!("$a").as_str())
        .to_string();
    output = n_blocks
        .replace_all(output.as_str(), bold!("$a").as_str())
        .to_string();
    output = n_allocs
        .replace_all(output.as_str(), bold!("$a").as_str())
        .to_string();
    output = n_frees
        .replace_all(output.as_str(), bold!("$a").as_str())
        .to_string();

    let labels = vec![
        "total heap usage:",
        "in use at exit:",
        "Command:",
        "definitely lost:",
        "indirectly lost:",
        "possibly lost:",
        "still reachable:",
        "suppressed:",
    ];

    for l in labels {
        output = output.replace(l, bold!(&color!(Color::GREEN, l)).as_str());
    }

    let sections = vec!["HEAP SUMMARY:", "LEAK SUMMARY:", "ERROR SUMMARY:"];

    for s in sections {
        output = output.replace(s, bold!(&color!(Color::BLUE, s)).as_str());
    }

    println!("{}", output);
}
