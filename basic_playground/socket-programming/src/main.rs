use std::env; // https://doc.rust-lang.org/std/env/index.html

#[macro_use]
extern crate log; // https://docs.rs/env_logger/0.7.1/env_logger/

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1);
    }

    let protocal: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];

    match protocal {
        "tcp" => match role {
            "server" => {
                // TODO implement
            }
            "client" => {
                // TODO implement
            }
            _ => {
                missing_role();
            }
        }
        "udp" => match role {
            "server" => {
                // TODO implement
            }
            "client" => {
                // TODO implement
            }
            _ => {
                missing_role();
            }
        }
        _ => {
            missing_role();
        }
    }
}

fn missing_role() {
    error!("Please specify server or client on the 2nd argument.");
}