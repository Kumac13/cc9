use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprint!("invaild number of arguments");
        process::exit(1);
    }

    let mut p = args[1].chars().peekable();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    while let Some(c) = p.next() {
        match c {
            '+' => {
                if let Some(c) = p.next() {
                    if c.is_digit(10) {
                        println!("  add rax, {}", c);
                    }
                } else {
                    eprint!("invaild character: {}", c);
                    process::exit(1);
                }
            }
            '-' => {
                if let Some(c) = p.next() {
                    if c.is_digit(10) {
                        println!("  sub rax, {}", c);
                    }
                } else {
                    eprint!("invaild character: {}", c);
                    process::exit(1);
                }
            }
            _ => {
                println!("  mov rax, {}", c);
            }
        }
    }
    println!("  ret");
}
