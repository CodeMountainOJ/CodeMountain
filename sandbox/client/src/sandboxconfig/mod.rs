#[derive(Debug)]
pub struct SandboxConfig {
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
    pub memory_limit: u16
}