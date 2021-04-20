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
    }

    pub fn set_sandbox_executable(&mut self, executable: &String) {
        if !std::path::Path::new(executable).exists() {
            self.sandbox_executable = String::clone(executable);
            self.check_is_initialized();
        }
        else
        {
            panic!("Sandbox executable: \"{}\" does not exist!", executable);
        }
    }

    
    pub fn set_source_file(&mut self, source_file: &String) {
        if !std::path::Path::new(source_file).exists() {
            self.source_file = String::clone(source_file);
            self.check_is_initialized();
        }
        else
        {
            panic!("Source file: \"{}\" does not exist!", source_file);
        }
    }

    
    pub fn set_input_file(&mut self, input_file: &String) {
        if !std::path::Path::new(input_file).exists() {
            self.input_file = String::clone(input_file);
            self.check_is_initialized();
        }
        else
        {
            panic!("Input file: \"{}\" does not exist!", input_file);
        }
    }

    pub fn output_file(&mut self, output_file: &String) {
        if !std::path::Path::new(output_file).exists() {
            self.output_file = String::clone(output_file);
            self.check_is_initialized();
        }
        else
        {
            panic!("Output file: \"{}\" does not exist!", output_file);
        }
    }
}
