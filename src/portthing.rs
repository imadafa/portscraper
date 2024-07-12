

pub mod scanner{

    use std::{net::TcpStream};
    pub fn scan(ip: &str) {
        let port_range: Vec<u32> = (1..10000).collect();
    
        let mut handles = vec![];
    
        let mut open_ports = Vec::new();
        let total_ports = port_range.len();
        let mut completed_ports = 0;
    
        let start_time = std::time::Instant::now();
    
        for port in port_range {
            let ip = ip.to_string();
            let handle = std::thread::spawn(move || {
                let connection_with_timeout = TcpStream::connect_timeout(
                    &format!("{}:{}", ip, port).parse().unwrap(),
                    std::time::Duration::from_secs(1),
                );
                match connection_with_timeout {
                    Ok(_) => {
                        Some(port)
                    }
                    Err(_) => {
                        None
                    }
                }
            });
            handles.push(handle);
            completed_ports += 1;
    
            if handles.len() == 1000 {
                for handle in handles {
                    if let Some(port) = handle.join().unwrap() {
                        open_ports.push(port);
                    }
                }
                handles = vec![];
            }
    
            if completed_ports % 100 == 0 {
                let percentage = (completed_ports as f64 / total_ports as f64) * 100.0;
                let elapsed_time = start_time.elapsed();
                let time_per_port = elapsed_time.as_secs_f64() / (completed_ports as f64);
                let remaining_ports = total_ports - completed_ports;
                let remaining_time_secs = (time_per_port * remaining_ports as f64) as u64;
                let remaining_time_mins = remaining_time_secs / 60;
    
                print!("\x1B[2J\x1B[1;1H");
                println!("{}% complete. Estimated time remaining: {} secs ({} mins)", percentage, remaining_time_secs, remaining_time_mins);
            }
        }
    
        for handle in handles {
            if let Some(port) = handle.join().unwrap() {
                open_ports.push(port);
            }
        }
    
        let elapsed_time = start_time.elapsed();
        println!("Scan completed in {}m {}s", elapsed_time.as_secs() / 60, elapsed_time.as_secs() % 60);
    
        if open_ports.is_empty() {
            println!("No open ports found.");
        } else {
            println!("STATE\tPORT\tPROTOCOL");
            for port in open_ports {
                println!("OPEN\t{}\tTCP", port);
            }
        }
    
    }

    pub fn ip(){
        println!("192.168.0.1(its not wrong)");
    }

    pub fn ping_g(){
        println!("some number!")
    }

    pub fn otherthing(){
        println!(" _______  _______  ______    _______  _______  _______  ______    _______  _______  _______  ______   
|       ||       ||    _ |  |       ||       ||       ||    _ |  |   _   ||       ||       ||    _ |  
|    _  ||   _   ||   | ||  |_     _||  _____||       ||   | ||  |  |_|  ||    _  ||    ___||   | ||  
|   |_| ||  | |  ||   |_||_   |   |  | |_____ |       ||   |_||_ |       ||   |_| ||   |___ |   |_||_ 
|    ___||  |_|  ||    __  |  |   |  |_____  ||      _||    __  ||       ||    ___||    ___||    __  |
|   |    |       ||   |  | |  |   |   _____| ||     |_ |   |  | ||   _   ||   |    |   |___ |   |  | |
|___|    |_______||___|  |_|  |___|  |_______||_______||___|  |_||__| |__||___|    |_______||___|  |_|");
        println!("BY ADAFA @imadafa on git")
    }
}