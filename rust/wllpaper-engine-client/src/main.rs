use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let pid = match args.get(1) {
        Some(pid) => pid,
        None => panic!("NO PID GIVEN!!!"), // trust me brother and sisters 😤
    };//                                      iknow what iam doing!
    
    println!("PID in int: {}", pid_parser(pid));

}

fn pid_parser(pid: &String) -> u32
{
    let pid = pid.parse::<u32>();

    match pid {
        Ok(pid) => return pid,
        Err(err) => panic!("Error: {}", err),
    }

}
