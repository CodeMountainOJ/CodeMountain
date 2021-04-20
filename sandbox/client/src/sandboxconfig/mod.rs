#[derive(Debug)]
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
    pub uninitialized: bool
}

impl SandboxConfig {
    pub fn new() -> Self {
        Self {
            sandbox_executable: String::from(""),
            source_file: String::from(""),
            input_file: String::from(""),
            output_file: String::from(""),
            compiler_output_file: String::from(""),
            compile_cmd: String::from(""),
            executable: String::from(""),
            executable_args: Vec::new(),
            uid: 0,
            gid: 0,
            time_limit: 0,
            memory_limit: 0,
            uninitialized: true
        }
    }

    fn check_is_initialized(&mut self) {
        if self.sandbox_executable.len() > 0 && self.source_file.len() > 0 && self.input_file.len() > 0 && self.output_file.len() > 0
            && self.compiler_output_file.len() > 0 && self.compile_cmd.len() > 0 && self.executable.len() > 0 && self.executable_args.len() > 0
            && self.uid > 0 && self.gid > 0 && self.time_limit > 0 && self.memory_limit > 0 {
                self.uninitialized = false;
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