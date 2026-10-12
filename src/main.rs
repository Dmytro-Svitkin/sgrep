use std::env;

#[derive(Debug)]
enum Command{
    New,Move,Remove,Read,Write,Rewrite,Replace,Copy,Permisson,List,Scan,
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
            
            _=>return Command::None,
        }
    }
    Command::None
}

fn resolve_invalid_command(invalid_command_str:&str){}// To be used only when the Command is known to be None.

fn is_path(path_str:&str)->bool{// Any path is valid as long as it starts and ends with double-quotation-marks (").
    return!path_str.is_empty()&&path_str.chars().nth(0).unwrap()=='"'&&path_str.chars().nth(path_str.len()).unwrap()=='"'
}

fn main(){
    for argument in get_arguments(){
        println!("{:?}",match_str_command(&argument))
    }
}