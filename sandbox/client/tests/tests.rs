use sandbox_client::sandboxconfig;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_test() {
        let _config = sandboxconfig::SandboxConfig::new();

        assert_eq!(_config.source_file, String::from(""));
        assert_eq!(_config.input_file, String::from(""));
        assert_eq!(_config.output_file, String::from(""));
        assert_eq!(_config.compiler_output_file, String::from(""));
        assert_eq!(_config.compile_cmd, String::from(""));
        assert_eq!(_config.executable, String::from(""));
        assert_eq!(_config.executable_args.len(), 0);
        assert_eq!(_config.uid, 0);
        assert_eq!(_config.gid, 0);
        assert_eq!(_config.time_limit, 0);
        assert_eq!(_config.memory_limit, 0);
        assert_eq!(_config.uninitialized, true);
    }
}