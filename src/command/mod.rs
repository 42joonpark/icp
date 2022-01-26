use anyhow::{Context, Result};

use crate::structs::program;

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

// pub async fn welcome_msg(prog: &mut program::Program) -> Result<(), Box<dyn error::Error>> {
// pub async fn welcome_msg(prog: &mut program::Program) -> Result<()> {
//     me::load_info(prog)
//         .await
//         .with_context(|| "Failed to load info".to_string())?;
//     clear();
//     println!("Welcome {}!", prog.me.login.to_owned());
//     Ok(())
// }

pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

/*
pub fn email(prog: &mut program::Program) {
    println!("Email: {}", prog.me.email);
}

pub fn id(prog: &mut program::Program) {
    println!("ID: {}", prog.me.id);
}

pub fn login(prog: &mut program::Program) {
    println!("Intra ID: {}", prog.me.login);
}

pub fn wallet(prog: &mut program::Program) {
    println!("Intra ID: {}", prog.me.wallet);
}

pub fn correction_point(prog: &mut program::Program) {
    println!("Correction Point: {}", prog.me.correction_point);
}

pub async fn reload_me(prog: &mut program::Program) -> Result<()> {
    println!("Reloading My Info");
    me::load_info(prog)
        .await
        .with_context(|| "Failed to load info".to_string())?;
    println!("Reloaded My Info.");
    Ok(())
}
*/

pub mod help;
pub mod me;
