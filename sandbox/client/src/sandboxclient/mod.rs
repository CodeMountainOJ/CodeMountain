/* 
 *  CodeMountain is a free and open source online judge open for everyone
 *  Copyright (C) 2021 MD Gaziur Rahman Noor and contributors
 *  
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *  
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *  
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::process::Command;

#[derive(Debug, Default)]
pub struct SandboxClient {
    pub sandbox_executable: String,
    pub source_file: String,
    pub input_file: String,
    pub output_file: String,
    pub compiler_output_file: String,
    pub compile_cmd: String,
    pub executable: String,
    pub executable_args: Vec<String>,
    pub uid: u16,
    pub gid: u16,
    pub time_limit: u16,
    pub memory_limit: u16,
    pub command: Vec<String>,
    pub initialized: bool,
    pub sandbox_status: serde_json::Value,
    pub sandbox_failed: bool
}

impl SandboxClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check_is_initialized(&mut self) {
        if self.sandbox_executable.len() > 0 && self.source_file.len() > 0 && self.input_file.len() > 0
        && self.output_file.len() > 0 && self.executable.len() > 0 &&
           self.uid != 0 && self.gid != 0 && self.time_limit != 0 && self.memory_limit != 0 {
            if self.compile_cmd.len() > 0 {
                if self.compiler_output_file.len() > 0 {
                    self.initialized = true
                } else {
                    self.initialized = false;
                }
            }
            else {
                self.initialized = true;
            }
        }
        else {
            self.initialized = false;
        }
    }

    pub fn set_sandbox_executable(&mut self, executable: &String) {
        match std::fs::read(executable) {
            Ok(_) => self.sandbox_executable = executable.clone(),
            Err(_) => panic!("Sandbox executable: \"{}\" does not exist!", executable)
        }
    }

    
    pub fn set_source_file(&mut self, source_file: &String) {
        match std::fs::read(source_file) {
            Ok(_) => self.source_file = source_file.clone(),
            Err(_) => panic!("Source file: \"{}\" does not exist!", source_file)
        }
    }

    
    pub fn set_input_file(&mut self, input_file: &String) {
        match std::fs::read(input_file) {
            Ok(_) => self.input_file = input_file.clone(),
            Err(_) => panic!("Input file: \"{}\" does not exist", input_file)
        }
    }

    pub fn set_output_file(&mut self, output_file: &String) {
        self.output_file = String::clone(output_file);
    }

    pub fn set_compiler_output_file(&mut self, compiler_output_file: &String) {
        if self.compile_cmd.len() == 0 {
            panic!("You need to set the compile command to set output file");
        }
        self.compiler_output_file = String::clone(compiler_output_file);
    }

    pub fn set_compile_cmd(&mut self, compiler_cmd: &String) {
        self.compile_cmd = String::clone(compiler_cmd);
    }

    pub fn set_executable(&mut self, executable: &String) {
        self.executable = String::clone(executable);
    }

    pub fn set_executable_args(&mut self, executable_args: &Vec<String>) {
        self.executable_args = executable_args.clone();
    }

    pub fn set_uid(&mut self, uid: u16) {
        if uid == 0 {
            panic!("UID cannot be the root one");
        }
        self.uid = uid;
    }

    pub fn set_gid(&mut self, gid: u16) {
        if gid == 0 {
            panic!("GID cannot be the root one");
        }
        self.gid  = gid;
    }

    pub fn set_time_limit(&mut self, time_limit: u16) {
        self.time_limit = time_limit;
    }

    pub fn set_memory_limit(&mut self, memory_limit: u16) {
        if memory_limit < 128 {
            panic!("Memory limit cannot be less than 128 MB");
        }
        self.memory_limit = memory_limit;
    }

    pub fn gen_cmd(&mut self) {
        self.check_is_initialized();
        if !self.initialized {
            panic!("You must set all the values as required to generate command");
        }

        self.command.push(String::from("-s"));
        self.command.push(self.source_file.clone());
        self.command.push(String::from("-i"));
        self.command.push(self.input_file.clone());
        self.command.push(String::from("-o"));
        self.command.push(self.output_file.clone());
        
        if self.compile_cmd.len() != 0 {
            self.command.push(String::from("-c"));
            self.command.push(self.compile_cmd.clone());
            self.command.push(String::from("-g"));
            self.command.push(self.compiler_output_file.clone());
        }

        self.command.push(String::from("-e"));
        self.command.push(self.executable.clone());
        self.command.push(String::from("-r"));
        self.command.push(self.executable_args.join(" "));

        self.command.push(String::from("-t"));
        self.command.push(format!("{}", self.time_limit.clone()));
        self.command.push(String::from("-m"));
        self.command.push(format!("{}", self.memory_limit.clone()));
        self.command.push(String::from("-u"));
        self.command.push(format!("{}", self.uid.clone()));
        self.command.push(String::from("-d"));
        self.command.push(format!("{}", self.gid.clone()));
    }

    pub fn run(&mut self)  {
        self.check_is_initialized();
        if !self.initialized {
            panic!("You need to initialize before calling this function");
        }
        else if self.command.len() == 0 {
            panic!("You need to generate command before calling this function");
        }

        let output = Command::new(self.sandbox_executable.clone())
                              .args(self.command.clone())
                              .output()
                              .expect("Failed to run sandbox process. This is probably a system error");

        if !output.status.success() {
            self.sandbox_failed = true;
            return;
        }

        match serde_json::from_str(&String::from_utf8_lossy(&output.stdout)) {
            Ok(parsed_json) => self.sandbox_status = parsed_json,
            Err(_) => self.sandbox_failed = true
        }
    }

    pub fn get_cmd(&mut self) -> Vec<String> {
        self.command.clone()
    }
}
