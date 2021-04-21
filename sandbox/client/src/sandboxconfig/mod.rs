/* 
 *  CodeMountain is a free and open source online judge open for everyone
 *  Copyright (C) 2021 MD Gaziur Rahman Noor
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
use crate::utils;


#[derive(PartialEq, Default, Debug)]
pub struct SandboxConfig {
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
    pub command: String,
    pub initialized: bool
}

impl SandboxConfig {
    pub fn new() -> Self {
        Self::default()
    }

    fn check_is_initialized(&mut self) {
        if self != &mut Self::new() {
            self.initialized = true;
        }

        if self.compile_cmd.len() > 0 {
            if self.compiler_output_file.len() > 0 {
                self.initialized = true
            } else {
                self.initialized = false;
            }
        }
    }

    pub fn set_sandbox_executable(&mut self, executable: &String) {
        if !std::path::Path::new(executable).exists() {
            self.sandbox_executable = String::clone(executable);
        }
        else
        {
            panic!("Sandbox executable: \"{}\" does not exist!", executable);
        }
    }

    
    pub fn set_source_file(&mut self, source_file: &String) {
        if !std::path::Path::new(source_file).exists() {
            self.source_file = String::clone(source_file);
        }
        else
        {
            panic!("Source file: \"{}\" does not exist!", source_file);
        }
    }

    
    pub fn set_input_file(&mut self, input_file: &String) {
        if !std::path::Path::new(input_file).exists() {
            self.input_file = String::clone(input_file);
        }
        else
        {
            panic!("Input file: \"{}\" does not exist!", input_file);
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
            panic!("You need to set all the values before you can generate command");
        }
        let command: String;
        if self.compile_cmd.len() > 0 {
            command = format!("{} -s {} -i {} -o {} -c \"{}\" -g \"{}\" -e \"{}\" -r \"{}\" -t {} -m {} -u {} -d {}", self.sandbox_executable, self.source_file, self.input_file, 
                                self.output_file, self.compile_cmd, self.compiler_output_file, self.executable, utils::join(self.executable_args.clone(), " "), 
                                self.time_limit, self.memory_limit, self.uid, self.gid);
        }
        else {
            command = format!("{} -s {} -i {} -o {} -e \"{}\" -r \"{}\" -t {} -m {} -u {} -d {}", self.sandbox_executable, self.source_file, self.input_file, 
                                self.output_file, self.executable, utils::join(self.executable_args.clone(), " "), 
                                self.time_limit, self.memory_limit, self.uid, self.gid);
        }

        self.command = command;
    }

    pub fn get_cmd(&mut self) -> String {
        self.command.clone()
    }
}
