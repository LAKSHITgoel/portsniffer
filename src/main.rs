use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::process;

const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough argments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let flag_or_ip = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&flag_or_ip) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!(
                    "Usage:\n -j: to select how many threads you want
                \r\n    -h or --help: to show helpfull message"
                );

                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be a valid IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number"),
                };

                return Ok(Arguments {
                    flag,
                    ipaddr,
                    threads,
                });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

fn scan(sender: Sender<u16>, start_port: u16, ipaddr: IpAddr, number_of_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((ipaddr, port)) {
            Ok(_) => {
                println!("\nHELL YEAH!!! \n");
                io::stdout().flush().unwrap();
                sender.send(port).unwrap();
            }
            Err(_) => {
                print!("HOLY MOLLY ");
            }
        }

        if (MAX - port) <= number_of_threads {
            break;
        }
        port += number_of_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments {}", program, err);
            process::exit(0);
        }
    });

    let number_of_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (sender, receiver) = channel();
    for i in 0..number_of_threads {
        let sender2 = sender.clone();
        thread::spawn(move || {
            // println!("creating a thread");
            scan(sender2, i, addr, number_of_threads)
        });
    }

    let mut out = vec![0];
    drop(sender);
    for i in receiver {
        out.push(i);
    }
    println!("");
    out.sort();
    for i in out {
        println!("{} is open", i);
    }
}
