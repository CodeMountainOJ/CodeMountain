use sandbox_client::sandboxconfig;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_test() {
        let config = sandboxconfig::SandboxConfig {
            source_file: String::from("program.c"),
            input_file: String::from("input.txt"),
            output_file: String::from("output.txt"),
            compiler_output_file: String::from("compiler_output.txt"),
            compile_cmd: String::from("/usr/bin/gcc program.c -oprogram -DONLINE_JUDGE"),
            executable: String::from("program"),
            executable_args: Vec::new(),
            uid: 1001,
            gid: 1001,
            time_limit: 1,
            memory_limit: 5
        };

        let empty_vector: Vec<String> = Vec::new();

        assert_eq!(config.source_file, "program.c");
        assert_eq!(config.input_file, "input.txt");
        assert_eq!(config.output_file, "output.txt");
        assert_eq!(config.compiler_output_file, "compiler_output.txt");
        assert_eq!(config.compile_cmd, "/usr/bin/gcc program.c -oprogram -DONLINE_JUDGE");
        assert_eq!(config.executable, "program");
        assert_eq!(config.executable_args, empty_vector);
        assert_eq!(config.uid, 1001);
        assert_eq!(config.gid, 1001);
        assert_eq!(config.time_limit, 1);
        assert_eq!(config.memory_limit, 5);
    }
}