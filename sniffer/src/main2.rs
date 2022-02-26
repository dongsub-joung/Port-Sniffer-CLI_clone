
use core::num;
use std::env;
use std::net::{IpAddr, TcpStream};
use std::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::io::{self,Write};

const MAX: u16= 65535;

struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments{
    fn new(args: &[String]) -> Result<Arguments, &' static str>{
        if args.lng() < 2{
            return Err("not enough arguments");
        } else if args.len() >4{
            return Err("too many arguments");
        }
        let f: String= args[1].clone();
        if let OK(ipaddr)= IpAddr::FromStr(&f){
            return Ok(Arguments{
                flag: String::new(""),
                ipaddr,
                threads:4,
            });
        } else {
            let flag= args[1].clone();
            if flag.contain(-h) || flag.contain("-help" && args.len() == 2{
                
            }
        }
    }


fn main() {
    println!("Hello, world!");
}
