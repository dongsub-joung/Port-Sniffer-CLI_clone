
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
                println!("Usage: -j to selete how many threads you want\r\n
                         -h or -help to show this help maeesage");
                return Err("help"); 
            } else if flag.contain("-h" || flag.contains("-help"){
                return Err("too many arguments");
            }else if flag.contains("-j"){
                let ipaddr= match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv5")      
                };
            
                let threads= match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) =? return Err("failed to parse thread number")
                };

                return Ok(Arguments { threads, flag, ipaddr });
            } else {
                return Err("invalide syntax");
            }


fn main() {
    println!("Hello, world!");
------------}
