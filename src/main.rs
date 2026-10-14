use std::{fs,env};

#[derive(Debug,PartialEq)]
enum Command{
    New,Move,Remove,Read,Write,Rewrite,Replace,Copy,Permisson,List,Scan,Execute,
    None
}

fn get_arguments()->Vec<String>{
    env::args().collect()
}

fn match_str_command(str_command:&str)->Command{
    if str_command.is_ascii(){
        match str_command.to_lowercase().as_str(){
            "new"=>       return Command::New,
            "move"=>      return Command::Move,
            "remove"=>    return Command::Remove,
            "read"=>      return Command::Read,
            "write"=>     return Command::Write,
            "rewrite"=>   return Command::Rewrite,
            "replace"=>   return Command::Replace,
            "copy"=>      return Command::Copy,
            "permission"=>return Command::Permisson,
            "perm"=>      return Command::Permisson,
            "list"=>      return Command::List,
            "scan"=>      return Command::Scan,
            "execute"=>   return Command::Execute,
            "exe"=>       return Command::Execute,
            
            "nw"=>return Command::New,
            "mv"=>return Command::Move,
            "rm"=>return Command::Remove,
            "rd"=>return Command::Read,
            "wr"=>return Command::Write,
            "rw"=>return Command::Rewrite,
            "rp"=>return Command::Replace,
            "cp"=>return Command::Copy,
            "pr"=>return Command::Permisson,
            "ls"=>return Command::List,
            "sc"=>return Command::Scan,
            "ex"=>return Command::Execute,
            "x"=> return Command::Execute,
            
            _=>return Command::None,
        }
    }
    Command::None
}

fn resolve_invalid_command(command_str:&str)->&str{// To be used only when the Command is known to be None.
    if match_str_command(command_str)==Command::None&&!is_path(command_str){panic!("[!] ILLEGAL INPUT")}
    else{command_str}
}

fn is_path(path_str:&str)->bool{// Any path is valid as long as it starts and ends with double-quotation-marks, while containing at lease 1 character (`"path/something"`).
    return path_str.len()>2&&path_str.chars().nth(0).unwrap()=='"'&&path_str.chars().last().unwrap()=='"'
}

fn main(){
    let arguments:Vec<String>=get_arguments();
    let mut file_path_argument:String=String::new();
    let mut index_flag:bool=false;
    
    for argument in arguments{
        if index_flag==true{file_path_argument=argument.clone();break}
        if match_str_command(&argument)==Command::Read{index_flag=true}
    }

    let content:String=fs::read_to_string(file_path_argument).expect("[!] FILE NOT FOUND");
    println!("{}",content)
}