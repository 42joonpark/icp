pub fn help() {
    println!("***** HELP ******");
    println!("1. Commands");
    println!("\temail: email");
    println!("\tid: personal id");
    println!("\tlogin: intra id");
    println!("\twallet: my wallet amount");
    println!("\tpoint: my correction point");
    println!("\treload: reload my information");
    println!("\tquit: quit program");
}

pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub mod me;
