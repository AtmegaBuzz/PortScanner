// simple port scanner using rust


use std::net::{TcpStream,IpAddr};
use std::env;
use std::str::FromStr;

struct Params{
    port: String,
    ipArrd: IpAddr
}

impl  Params{

    fn validate_port(port: &str) -> Result<&str, &'static str>{
        if port.chars().nth(0).unwrap()=='-'{
            return Ok("all")
        }

        for chr in port.chars(){
            if !chr.is_numeric() {
                return Err("invalid port.");
            }
        }

        return Ok(port);

    }

    fn validate_ipAddr(ip: &str) -> Result<IpAddr,&'static str> {
        if ip.chars().nth(0).unwrap()=='-'{
            return Err("Ip address cannot be empty");
        }

        match IpAddr::from_str(&ip) {
            Ok(vipaddr) => return Ok(vipaddr),
            Err(_) => return Err("not a valid ip address")
        };

    }

    pub fn new(args: &Vec<String>) -> Result<Params, &'static str>{
        if args.len() < 2{
            return Err("less arguments passed");
        } else if args.len() > 4 {
            return Err("Too many arguments passed");
        }

        let idx_port : usize = args.iter().position(|r| r =="-p").unwrap();
        let mut initPortVal: &str = &args[idx_port+1].clone();

        let idx_ip : usize = args.iter().position(|r| r == "-ip").unwrap();
        let mut initIpAddrVal: &str = &args[idx_ip+1].clone(); 
        
        // clean port
        let port: &str = match self::Params::validate_port(initPortVal){
            Ok(port_val) => port_val,
            Err(err_val) => return Err(err_val)
        };

        // clean ip 

        let ipAddr: IpAddr = match self::Params::validate_ipAddr(initIpAddrVal) {
            Ok(ip_val) => ip_val,
            Err(err_val) => return Err(err_val)
        };

        Ok(
            Params{
                port: port.to_string(),
                ipArrd: ipAddr
            }
        )

        

    }
}


fn main(){
    let args: Vec<_> = env::args().collect();

}