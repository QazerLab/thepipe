use std::env;
use std::io::BufRead;
use std::io::Write;
use std::process::exit;
use std::process::ChildStdin;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let thepipe_args: Vec<String> = env::args().skip(1).collect();
    let subproc_args: Vec<String> = env::args().skip(2).collect();

    if thepipe_args.is_empty() {
        eprintln!("No command to pipe into was specified!");
        exit(1);
    }

    let cmd = Command::new(&thepipe_args[0])
        .stdin(Stdio::piped())
        .args(&subproc_args)
        .spawn();

    let mut child = match cmd {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not spawn a child: {}", e);
            exit(1);
        }
    };

    match child.stdin {
        Some(ref mut stream) => {
            print_header_unchanged();
            stream_stdin_to(stream);
        }

        None => {
            eprintln!("Could not capture subprocess' stdin!");
            exit(1);
        }
    };

    match child.wait() {
        Ok(result) => {
            if !result.success() {
                exit(2);
            }
        }

        Err(e) => {
            eprintln!("Failed waiting for subprocess: {}", e);
            exit(1);
        }
    }
}

fn stream_stdin_to(sub_stdin: &mut ChildStdin) {
    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(ln) => {
                if let Err(e) = writeln!(sub_stdin, "{}", ln) {
                    eprintln!("Failed writing to child process: {}", e);
                    exit(1);
                }
            }

            Err(e) => {
                eprintln!("Failed reading stdin: {}", e);
                exit(1);
            }
        }
    }
}

fn print_header_unchanged() {
    let mut buf = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut buf) {
        eprintln!("Failed reading the header line: {}", e);
    }

    print!("{}", buf);
}
