#[derive(Debug)]
pub enum Command {
    Unknown(String),
    Exit(),
    Add{
        user: String,
        dep: String,
    },
    Search(String),
    Dump(),
    Warn{
        command: String,
        reason: String,
    }
}

pub fn parse_command(raw_cmd: &String) -> Command {
    let words: Vec<&str> = raw_cmd.split(" ").collect();
    let empty_str = "";
    let fw = words.get(0).unwrap_or(&empty_str);
    let fw = fw.to_lowercase();
    let fw = fw.trim();
    //let fw = fw.as_str(); // would get the string version if I hadn't trimmed it.
    let cmd = match fw {
        "add" => {
            if words.len() >= 3 {
                Command::Add{
                    user: words[1].trim().to_string(),
                    dep: words[2].trim().to_string(),
                }
            } else {
                Command::Warn{
                    command: "add".to_string(),
                    reason: "insufficent paramaters".to_string(),
                }
            }
        },
        "department" | "dep" | "list" => {
            if words.len() >= 2 {
                Command::Search(words[1].trim().to_string())
            } else {
                Command::Warn{
                    command: "department".to_string(),
                    reason: "insufficent paramaters".to_string(),
                }
            }
        }
        "all" | "dump" => {
            Command::Dump()
        }
        "exit" => {
            Command::Exit()
        }
        other => {
            Command::Unknown(other.to_string())
        }
    };
    cmd
}

#[allow(dead_code)]
pub fn help_all() {

}

#[allow(dead_code)]
pub fn help(command: String, problem: Option<String>){
    println!("Stubby help for command {} with problem {:?}", command, problem);
}