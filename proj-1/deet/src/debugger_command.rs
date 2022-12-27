pub enum DebuggerCommand {
    Backtrace,
    Breakpoint(String),
    Continue,
    Quit,
    Run(Vec<String>),
}

impl DebuggerCommand {
    pub fn from_tokens(tokens: &Vec<&str>) -> Option<DebuggerCommand> {
        match tokens[0] {
            "bt" | "back" | "backtrace" => Some(DebuggerCommand::Backtrace),
            "b" | "break" | "breakpoint" => {
                if tokens.len() != 2 {
                    println!("Usage: breakpoint <*hexadecimal address>/<function>/<line number>");
                    None
                } else {
                    Some(DebuggerCommand::Breakpoint(tokens[1].to_string()))
                }
            }
            "c" | "cont" | "continue" => Some(DebuggerCommand::Continue),
            "q" | "quit" => Some(DebuggerCommand::Quit),
            "r" | "run" => {
                let args = tokens[1..].to_vec();
                Some(DebuggerCommand::Run(
                    args.iter().map(|s| s.to_string()).collect(),
                ))
            }
            // Default case:
            _ => None,
        }
    }
}
