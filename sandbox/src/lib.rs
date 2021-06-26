/// # The Sandbox struct
/// ## Use this to configure the sandbox
/// ## Calling the run function will start the actual sandboxing stuff

#[derive(Debug, Default)]
pub struct Sandbox {
    /// Max CPU time(ms)
    cpu_time_limit: u32,

    /// Max real time(ms)
    real_time_limit: u32,

    /// Memory limit in kilobytes
    memory_limit: u32,

    /// Stack limit
    stack_limit: u32,

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

    pub fn set_cpu_time_limit(&mut self, cpu_time: u32) {
        self.cpu_time_limit = cpu_time;
    }

    pub fn set_real_time_limit(&mut self, real_time: u32) {
        self.real_time_limit = real_time;
    }
}

struct SandboxResult {
    // Real time spent by child process
    pub real_time_spent: u32,

    /// CPU time spent by child process
    pub cpu_time_spent: u32,
    
    /// Memory spent by child process
    pub memory_limit: u32,

    /// Signal returned by child process
    pub signal: u32,

    /// Return code of child process
    pub return_code: u32
}