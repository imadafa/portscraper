mod portthing;
use crate:: portthing::scanner::scan;
use crate:: portthing::scanner::ip;
use crate:: portthing::scanner::ping_g;
use crate::portthing::scanner::otherthing;
use std::io;


fn main(){
    otherthing();
   loop{
    
    
    
    println!("\n \r[-ip] Show IP\n [-s] PortScan\n [-pg] Ping google \n [-h] help");
    
    let mut input = String::new();

    io::stdin()
     .read_line(&mut input)
     .expect("ENTER A NUMBER");

    let input: &str = &input.trim();
    match input{
        "-h" => println!("HELP COSTS MONEY BROKEY!"),
        "-s" => scan("192.168.0.1"),
        "-pg" => ping_g(),
        "-ip" => ip(),
        &_ => todo!()
    }
    //break;
    

   }
}
  