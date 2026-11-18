#![allow(non_snake_case)]
use std::fs;
use std::fs::File;
use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
fn main() {
    let aRgs_nya: Vec<String> = env::args().skip(1).collect();
    match aRgs_nya.as_slice() {
        [cmd, name] if cmd == "folder" => folder(name),
        [cmd, name] if cmd == "new" => arch(name),
        [cmd] if cmd == "cmd" => println!("cmd\nfolder\ngoto (Only works on terminal mode)\nnew\nfls\nerase\ndelDir"),
        [cmd] if cmd == "fls" => list(),
        [cmd, pth] if cmd == "delDir" => rem(pth),
        [cmd, fl] if cmd == "erase" => eras(fl),
        [] => ter(),
        _ => println!("Command not found: {:?}", aRgs_nya)
    }
    return ();
}
fn ter(){
    println!("=======TERMINAL MODE=======");
    println!("=====TYPE THE COMMANDS=====");
    loop {
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Command expected");
        let args: Vec<&str> = cmd
            .split_whitespace()
            .collect();
        match args.as_slice() {
            ["folder", name] => folder(&name.to_string()),
            ["new", name] => arch(&name.to_string()),
            ["goto", pth] => goto(&pth.to_string()),
            ["delDir", pth] => rem(&pth.to_string()),
            ["erase", name] => eras(&name.to_string()),
            ["end"] => break,
            ["fls"] => list(),
            _ => println!("folder\ngoto\nnew\nfls\nend\nerase\ndelDir")
        }
    }
}
fn folder(name: &String){
    let pt = Path::new(name);
    if pt.is_dir(){
        println!("Already exists");
    }
    else {
        fs::create_dir(name).expect("Couldn't create the directory");
    }
}
fn arch(name: &String){
    match arcC(name) {
        Ok(mut fil) => {
            let mut txt = String::new();
            println!("Write the archive content below (Ctrl+X to finish)");
            loop {
                let mut line = String::new();
                match io::stdin().read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        if let Some(end) = line.find('\u{18}') {
                            txt.push_str(&line[..end]);
                            break;
                        }
                        txt.push_str(&line);
                    }
                    Err(er) => {
                        println!("Couldn't read the archive content due to '{er}'");
                        return;
                    }
                }
            }
            fil.write_all(txt.as_bytes()).expect("Couldn't write your file");
        },
        Err(er) => println!("Couldn't create the file due to '{er}'")
    }
}
fn arcC(name: &String) -> io::Result<File>{
    let fl = File::create(name)?;
    Ok(fl)
}
fn goto(path: &String){
    match gotoErr(path) {
        Ok(()) => return,
        Err(er) => println!("Couldn't change the directory due to {er}")
    }
}
fn gotoErr(pth: &String) -> io::Result<()>{
    env::set_current_dir(pth).expect("");
    Ok(())
}
fn list(){
    let pth = match env::current_dir() {
        Ok(p) => p,
        Err(er) =>{
            println!("Couldn't get the current directory: {er}");
            return;
        }
    };
    match fs::read_dir(pth) {
        Ok(rs) => {
            for ent in rs {
                match ent {
                    Ok(en) => println!("{}", en.path().display()),
                    Err(er) => println!("Couldn't read the directory due to {er}")
                }
            }
        }
        Err(er) => println!("Nothing found due to {er}")
    }
}
fn rem(pth: &String){
    println!("Wanna remove everything on the directory?[y/n]");
    let mut yn = String::new();
    io::stdin()
        .read_line(&mut yn)
        .expect("INVALID INPUT");
    let all = match yn.to_lowercase().as_str().trim() {
        "yes" => true,
        "y" => true,
        _ =>  false
    };
    if all{
        std::fs::remove_dir_all(pth)
            .expect("Couldn't see the dir");
    }
    else {
        std::fs::read_dir(pth)
            .expect("Couldn't see the dir: {err}");
    }
}
fn eras(name: &String){
    println!("are you sure you want to remove {name}?[y/n]");
    let mut chs = String::new();
    io::stdin()
        .read_line(&mut chs)
        .expect("Couldn't read");
    let all = match chs.to_lowercase().as_str().trim() {
        "yes" => true,
        "y" => true,
        _ =>  false
    };
    if all{
        std::fs::remove_file(name)
            .expect("Couldn't remove");
    }
    return;
}