use internal::{file::{create_file, file_exists}, runner::internal_run};

mod internal;

/// # The Sandbox struct
/// ## Use this to configure the sandbox
/// ## Calling the run function will start the actual sandboxing stuff
/// ### Use builder pattern to initiate the sandbox
#[derive(Debug, Default)]
pub struct Sandbox {
    /// Max CPU time(ms)
    cpu_time_limit: i64,

    /// Max real time(ms)
    real_time_limit: i64,

    /// Memory limit in kilobytes
    memory_limit: i64,

    /// Stack limit in kilobytes
    stack_limit: i64,

    /// Executable path
    executable_path: String,

    /// Executable's arguments
    executable_args: Vec<String>,

    /// File to replace the stdin of the sandboxed program with
    executable_stdin: String,

    /// File to replace the stdout of the sandboxed program with
    executable_stdout: String,

    /// File to replace the stderr of the sandboxed program with
    executable_stderr: String
}

impl Sandbox {
    pub fn new() -> Sandbox {
        Sandbox::default()
    }

    pub fn set_cputime_limit(&mut self, cpu_time: i64) -> &mut Self {
        self.cpu_time_limit = cpu_time;
        
        self
    }

    pub fn set_realtime_limit(&mut self, real_time: i64) -> &mut Self {
        self.real_time_limit = real_time;

        self
    }

    pub fn set_memory_limit(&mut self, memory_limit: i64) -> &mut Self {
        self.memory_limit = memory_limit;

        self
    }

    pub fn set_stack_limit(&mut self, stack_limit: i64) -> &mut Self {
        self.stack_limit = stack_limit;

        self
    }

    pub fn set_executable_path(&mut self, path: &str) -> &mut Self {
        if file_exists(path) {
            self.executable_path = path.to_string();
        }
        else {
            panic!("File {} doesn't exist", path);
        }
        
        self
    }

    pub fn set_executable_args(&mut self, args: &Vec<String>) -> &mut Self {
        self.executable_args = args.clone();

        self
    }

    pub fn set_executable_stdin(&mut self, path: &str) -> &mut Self {
        if file_exists(path) {
            self.executable_stdin = path.to_string();
        }
        else {
            panic!("File {} doesn't exist", path);
        }

        self
    }

    pub fn set_executable_stdout(&mut self, path: &str) -> &mut Self {
        if create_file(path) {
            self.executable_stdout = path.to_string();
        }
        else {
            panic!("Failed to create file {}", path);
        }

        self
    }

    pub fn set_executable_stderr(&mut self, path: &str) -> &mut Self {
        if create_file(path) {
            self.executable_stderr = path.to_string();
        }
        else {
            panic!("Failed to create file {}", path);
        }

        self
    }


    /// Runs the executable at given path in a secure environment
    /// Make sure you provided all the parameters correctly as it doesn't check if they're set or not
    pub fn run(self) -> SandboxResult {
        internal_run(self)
    }
}

#[derive(Default)]
pub struct SandboxResult {
    // Real time spent by child process
    pub real_time_spent: i64,

    /// CPU time spent by child process
    pub cpu_time_spent: i64,
    
    /// Memory spent by child process
    pub memory_spent: i64,

    /// Signal returned by child process
    pub signal: i32,

    /// Return code of child process
    pub return_code: i32,

    /// System Error?
    pub system_error: bool,

    /// Runtime Error?
    pub runtime_error: bool,

    /// CLE?
    pub cle: bool,

    /// TLE?
    pub tle: bool,
    
    /// MLE?
    pub mle: bool
}