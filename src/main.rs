use std::{
    env, fs,
    io::{stdin, stdout, Write},
};

use brainfuck::{executer::execute, parser::parse};

fn from_file(args: Vec<String>) {
    let file_path = args.get(1).unwrap();
    let src = fs::read_to_string(file_path).expect("Unable to read a file");

    let ast = parse(src.as_str());
    execute(ast.0);
}

fn from_stdin() {
    loop {
        print!("\n> ");
        stdout().flush().unwrap();
        let mut src = String::new();
        stdin().read_line(&mut src).unwrap();

        if src.contains("exit") {
            println!("Bye!");
            break;
        }

        let ast = parse(src.as_str());
        execute(ast.0);
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        from_file(args);
    } else {
        from_stdin();
    }
}
