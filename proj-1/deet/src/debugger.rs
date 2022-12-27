use crate::debugger_command::DebuggerCommand;
use crate::dwarf_data::{DwarfData, Error as DwarfError};
use crate::inferior::{Inferior, Status};
use rustyline::Editor;
use rustyline::error::ReadlineError;

fn parse_address(addr: &str) -> Option<usize> {
    if !addr.starts_with('*') {
        return None;
    }

    let addr_without_0x = if addr.to_lowercase().starts_with("*0x") {
        &addr[3..]
    } else {
        &addr[1..]
    };
    usize::from_str_radix(addr_without_0x, 16).ok()
}

pub struct Debugger {
    breakpoints: Vec<usize>,
    debug_data: DwarfData,
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        // TODO (milestone 3): initialize the DwarfData
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            },
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not load debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };

        debug_data.print();

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        Debugger {
            breakpoints: Vec::new(),
            debug_data,
            history_path,
            inferior: None,
            readline,
            target: target.to_string(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Backtrace => {
                    if let Some(inferior) = &self.inferior {
                        inferior.print_backtrace(&self.debug_data).unwrap();
                    } else {
                        println!("There is no inferior running.");
                    }
                },
                DebuggerCommand::Breakpoint(arg) => {
                    let mut addr = parse_address(&arg);
                    if addr.is_none() {
                        addr = self.debug_data.get_addr_for_function(None, &arg);
                    }
                    if addr.is_none() {
                        addr = self.debug_data.get_addr_for_line(None, arg.parse().unwrap_or(0));
                    }
                    if let Some(addr) = addr {
                        println!("Set breakpoint {} at {}", self.breakpoints.len(), addr);
                        self.breakpoints.push(addr);
                        if let Some(inferior) = &mut self.inferior {
                            inferior.set_breakpoint(addr).unwrap();
                        }
                    } else {
                        println!("Invalid argument.");
                    }
                },
                DebuggerCommand::Continue => {
                    if self.inferior.is_none() {
                        println!("There is no inferior running.");
                    } else {
                        self.inferior_continue_exec();
                    }
                },
                DebuggerCommand::Run(args) => {
                    if let Some(inferior) = &mut self.inferior {
                        inferior.kill().expect("Failed to kill the former inferior.");
                    }

                    if let Some(mut inferior) = Inferior::new(&self.target, &args) {
                        for breakpoint in &self.breakpoints {
                            inferior.set_breakpoint(*breakpoint).expect("Failed to set breakpoint");
                        }
                        self.inferior = Some(inferior);
                        self.inferior_continue_exec();
                    } else {
                        println!("Unable to start subprocess");
                    }
                }
                DebuggerCommand::Quit => {
                    if let Some(inferior) = &mut self.inferior {
                        inferior.kill().expect("Failed to kill inferior");
                    }
                    return;
                }
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().is_empty() {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }

    fn inferior_continue_exec(&mut self) {
        if let Some(inferior) = &mut self.inferior {
            match inferior.continue_exec() {
                Ok(status) => {
                    match status {
                        Status::Stopped(signal, rip) => {
                            let line = self.debug_data.get_line_from_addr(rip).unwrap();
                            let function = self.debug_data.get_function_from_addr(rip).unwrap();
                            println!("Child stopped (signal {})", signal);
                            println!("Stopped at {}:{}", line.file, line.number);
                            println!("In function `{}'", function);
                        },
                        Status::Exited(status) => {
                            self.inferior = None;
                            println!("Child exited (signal {})", status);
                        },
                        Status::Signaled(signal) => {
                            self.inferior = None;
                            println!("Child signaled (signal {})", signal);
                        },
                    }
                }
                Err(err) => {
                    println!("Inferior cannot be executed: {}", err);
                }
            }
        } else {
            println!("There is no inferior.");
        }
    }

    // fn set_breakpoint(&mut self, addr: usize) {
    //     if let Some(inferior) = &mut self.inferior {
    //         inferior.set_breakpoint(addr).expect("Failed to set breakpoint");
    //     }
    // }
}
