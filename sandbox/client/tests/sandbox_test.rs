use sandbox_client::sandboxclient;
use std::io::Read;

#[test]
fn output_test() {
    let uid = std::env::var("SANDBOX_UID").expect("Failed to get uid from env").parse::<u16>().unwrap();
    let gid = std::env::var("SANDBOX_GID").expect("Failed to get gid from env").parse::<u16>().unwrap();

    let mut config = sandboxclient::SandboxClient::new();
    config.set_sandbox_executable(&String::from("../build/codemountain_sandbox"));
    config.set_source_file(&String::from("../testprograms/rm/rm.c"));
    config.set_input_file(&String::from("../testprograms/rm/input.txt"));
    config.set_output_file(&String::from("../build/output.txt"));
    config.set_compile_cmd(&String::from(
        "/usr/bin/gcc ../testprograms/rm/rm.c -DONLINE_JUDGE -o../build/program",
    ));
    config.set_compiler_output_file(&String::from("../build/compiler_output_file.txt"));
    config.set_executable_args(&Vec::new());
    config.set_executable(&String::from("../build/program"));
    config.set_uid(uid);
    config.set_gid(gid);
    config.set_time_limit(1);
    config.set_memory_limit(128);
    config.gen_cmd();

    // execute
    config.run();

    let test_value: serde_json::Value = serde_json::from_str(
        r#"{
            "compileError": 0,
            "memoryLimitExceeded": 0,
            "runtimeError": 0,
            "spentTime": 0,
            "systemError": 0,
            "timeLimitExceeded": 0,
            "usedMemory": 2
          }"#,
    )
    .expect("Error occured while parsing test json data. This shouldn't happen");

    assert_eq!(config.sandbox_failed, false);
    assert_eq!(
        config.sandbox_status["compileError"],
        test_value["compileError"]
    );
    assert_eq!(
        config.sandbox_status["memoryLimitExceeded"],
        test_value["memoryLimitExceeded"]
    );
    assert_eq!(
        config.sandbox_status["runtimeError"],
        test_value["runtimeError"]
    );
    assert_eq!(config.sandbox_status["spentTime"], test_value["spentTime"]);
    assert_eq!(
        config.sandbox_status["systemError"],
        test_value["systemError"]
    );
    assert_eq!(
        config.sandbox_status["timeLimitExceeded"],
        test_value["timeLimitExceeded"]
    );
}

#[test]
fn testprogram_execve() {
    let uid = std::env::var("SANDBOX_UID").expect("Failed to get uid from env").parse::<u16>().unwrap();
    let gid = std::env::var("SANDBOX_GID").expect("Failed to get gid from env").parse::<u16>().unwrap();

    let mut config = sandboxclient::SandboxClient::new();
    config.set_sandbox_executable(&String::from("../build/codemountain_sandbox"));
    config.set_source_file(&String::from("../testprograms/execve/execve.c"));
    config.set_input_file(&String::from("../testprograms/rm/input.txt"));
    config.set_output_file(&String::from("../build/output.txt"));
    config.set_compile_cmd(&String::from(
        "/usr/bin/gcc ../testprograms/execve/execve.c -DONLINE_JUDGE -o../build/program",
    ));
    config.set_compiler_output_file(&String::from("../build/compiler_output_file.txt"));
    config.set_executable_args(&Vec::new());
    config.set_executable(&String::from("../build/program"));
    config.set_uid(uid);
    config.set_gid(gid);
    config.set_time_limit(1);
    config.set_memory_limit(128);
    config.gen_cmd();

    // execute
    config.run();

    let mut program_output = match std::fs::File::open("../build/output.txt") {
        Err(why) => panic!("Could not read program output file: {}", why),
        Ok(file) => file,
    };

    let mut program_output_data = String::new();
    match program_output.read_to_string(&mut program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let mut expected_program_output =
        match std::fs::File::open("../testprograms/execve/expected_output.txt") {
            Err(why) => panic!("Could not read program output file: {}", why),
            Ok(file) => file,
        };

    let mut expected_program_output_data = String::new();
    match expected_program_output.read_to_string(&mut expected_program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let test_value: serde_json::Value = serde_json::from_str(
        r#"{
            "compileError": 0,
            "memoryLimitExceeded": 0,
            "runtimeError": 0,
            "spentTime": 0,
            "systemError": 0,
            "timeLimitExceeded": 0,
            "usedMemory": 2
          }"#,
    )
    .expect("Error occured while parsing test json data. This shouldn't happen");

    assert_eq!(config.sandbox_failed, false);
    assert_eq!(
        config.sandbox_status["compileError"],
        test_value["compileError"]
    );
    assert_eq!(
        config.sandbox_status["memoryLimitExceeded"],
        test_value["memoryLimitExceeded"]
    );
    assert_eq!(
        config.sandbox_status["runtimeError"],
        test_value["runtimeError"]
    );
    assert_eq!(config.sandbox_status["spentTime"], test_value["spentTime"]);
    assert_eq!(
        config.sandbox_status["systemError"],
        test_value["systemError"]
    );
    assert_eq!(
        config.sandbox_status["timeLimitExceeded"],
        test_value["timeLimitExceeded"]
    );
    assert_eq!(program_output_data, expected_program_output_data);
}

#[test]
fn testprogram_python_helloworld() {
    let uid = std::env::var("SANDBOX_UID").expect("Failed to get uid from env").parse::<u16>().unwrap();
    let gid = std::env::var("SANDBOX_GID").expect("Failed to get gid from env").parse::<u16>().unwrap();
    
    let mut config = sandboxclient::SandboxClient::new();
    config.set_sandbox_executable(&String::from("../build/codemountain_sandbox"));
    config.set_source_file(&String::from("../testprograms/python-helloworld/program.py"));
    config.set_input_file(&String::from("../testprograms/rm/input.txt"));
    config.set_output_file(&String::from("../build/output.txt"));
    config.set_executable_args(&vec![String::from("../testprograms/python-helloworld/program.py")]);
    config.set_executable(&String::from("/usr/bin/python"));
    config.set_uid(uid);
    config.set_gid(gid);
    config.set_time_limit(1);
    config.set_memory_limit(128);
    config.gen_cmd();

    // execute
    config.run();

    let mut program_output = match std::fs::File::open("../build/output.txt") {
        Err(why) => panic!("Could not read program output file: {}", why),
        Ok(file) => file,
    };

    let mut program_output_data = String::new();
    match program_output.read_to_string(&mut program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let mut expected_program_output =
        match std::fs::File::open("../testprograms/python-helloworld/expected_output.txt") {
            Err(why) => panic!("Could not read program output file: {}", why),
            Ok(file) => file,
        };

    let mut expected_program_output_data = String::new();
    match expected_program_output.read_to_string(&mut expected_program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let test_value: serde_json::Value = serde_json::from_str(
        r#"{
            "compileError": 0,
            "memoryLimitExceeded": 0,
            "runtimeError": 0,
            "spentTime": 0,
            "systemError": 0,
            "timeLimitExceeded": 0,
            "usedMemory": 2
          }"#,
    )
    .expect("Error occured while parsing test json data. This shouldn't happen");

    assert_eq!(config.sandbox_failed, false);
    assert_eq!(
        config.sandbox_status["compileError"],
        test_value["compileError"]
    );
    assert_eq!(
        config.sandbox_status["memoryLimitExceeded"],
        test_value["memoryLimitExceeded"]
    );
    assert_eq!(
        config.sandbox_status["runtimeError"],
        test_value["runtimeError"]
    );
    assert_eq!(config.sandbox_status["spentTime"], test_value["spentTime"]);
    assert_eq!(
        config.sandbox_status["systemError"],
        test_value["systemError"]
    );
    assert_eq!(
        config.sandbox_status["timeLimitExceeded"],
        test_value["timeLimitExceeded"]
    );
    assert_eq!(program_output_data, expected_program_output_data);
}

#[test]
fn testprogram_python_input() {
    let uid = std::env::var("SANDBOX_UID").expect("Failed to get uid from env").parse::<u16>().unwrap();
    let gid = std::env::var("SANDBOX_GID").expect("Failed to get gid from env").parse::<u16>().unwrap();
    
    let mut config = sandboxclient::SandboxClient::new();
    config.set_sandbox_executable(&String::from("../build/codemountain_sandbox"));
    config.set_source_file(&String::from("../testprograms/python-input/program.py"));
    config.set_input_file(&String::from("../testprograms/python-input/input.txt"));
    config.set_output_file(&String::from("../build/output.txt"));
    config.set_executable_args(&vec![String::from("../testprograms/python-input/program.py")]);
    config.set_executable(&String::from("/usr/bin/python"));
    config.set_uid(uid);
    config.set_gid(gid);
    config.set_time_limit(1);
    config.set_memory_limit(128);
    config.gen_cmd();

    // execute
    config.run();

    let mut program_output = match std::fs::File::open("../build/output.txt") {
        Err(why) => panic!("Could not read program output file: {}", why),
        Ok(file) => file,
    };

    let mut program_output_data = String::new();
    match program_output.read_to_string(&mut program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let mut expected_program_output =
        match std::fs::File::open("../testprograms/python-input/expected_output.txt") {
            Err(why) => panic!("Could not read program output file: {}", why),
            Ok(file) => file,
        };

    let mut expected_program_output_data = String::new();
    match expected_program_output.read_to_string(&mut expected_program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let test_value: serde_json::Value = serde_json::from_str(
        r#"{
            "compileError": 0,
            "memoryLimitExceeded": 0,
            "runtimeError": 0,
            "spentTime": 0,
            "systemError": 0,
            "timeLimitExceeded": 0,
            "usedMemory": 2
          }"#,
    )
    .expect("Error occured while parsing test json data. This shouldn't happen");

    assert_eq!(config.sandbox_failed, false);
    assert_eq!(
        config.sandbox_status["compileError"],
        test_value["compileError"]
    );
    assert_eq!(
        config.sandbox_status["memoryLimitExceeded"],
        test_value["memoryLimitExceeded"]
    );
    assert_eq!(
        config.sandbox_status["runtimeError"],
        test_value["runtimeError"]
    );
    assert_eq!(config.sandbox_status["spentTime"], test_value["spentTime"]);
    assert_eq!(
        config.sandbox_status["systemError"],
        test_value["systemError"]
    );
    assert_eq!(
        config.sandbox_status["timeLimitExceeded"],
        test_value["timeLimitExceeded"]
    );
    assert_eq!(program_output_data, expected_program_output_data);
}

#[test]
fn testprogram_fs() {
    let uid = std::env::var("SANDBOX_UID").expect("Failed to get uid from env").parse::<u16>().unwrap();
    let gid = std::env::var("SANDBOX_GID").expect("Failed to get gid from env").parse::<u16>().unwrap();
    
    let mut config = sandboxclient::SandboxClient::new();
    config.set_sandbox_executable(&String::from("../build/codemountain_sandbox"));
    config.set_source_file(&String::from("../testprograms/fs/fs.c"));
    config.set_input_file(&String::from("../testprograms/fs/input.txt"));
    config.set_output_file(&String::from("../build/output.txt"));
    config.set_compile_cmd(&String::from(
        "/usr/bin/gcc ../testprograms/fs/fs.c -DONLINE_JUDGE -o../build/program",
    ));
    config.set_compiler_output_file(&String::from("../build/compiler_output_file.txt"));
    config.set_executable_args(&Vec::new());
    config.set_executable(&String::from("../build/program"));
    config.set_uid(uid);
    config.set_gid(gid);
    config.set_time_limit(1);
    config.set_memory_limit(128);
    config.gen_cmd();

    // execute
    config.run();

    let mut program_output = match std::fs::File::open("../build/output.txt") {
        Err(why) => panic!("Could not read program output file: {}", why),
        Ok(file) => file,
    };

    let mut program_output_data = String::new();
    match program_output.read_to_string(&mut program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let mut expected_program_output =
        match std::fs::File::open("../testprograms/fs/expected_output.txt") {
            Err(why) => panic!("Could not read program output file: {}", why),
            Ok(file) => file,
        };

    let mut expected_program_output_data = String::new();
    match expected_program_output.read_to_string(&mut expected_program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let test_value: serde_json::Value = serde_json::from_str(
        r#"{
            "compileError": 0,
            "memoryLimitExceeded": 0,
            "runtimeError": 0,
            "spentTime": 0,
            "systemError": 0,
            "timeLimitExceeded": 0,
            "usedMemory": 2
          }"#,
    )
    .expect("Error occured while parsing test json data. This shouldn't happen");

    assert_eq!(config.sandbox_failed, false);
    assert_eq!(
        config.sandbox_status["compileError"],
        test_value["compileError"]
    );
    assert_eq!(
        config.sandbox_status["memoryLimitExceeded"],
        test_value["memoryLimitExceeded"]
    );
    assert_eq!(
        config.sandbox_status["runtimeError"],
        test_value["runtimeError"]
    );
    assert_eq!(config.sandbox_status["spentTime"], test_value["spentTime"]);
    assert_eq!(
        config.sandbox_status["systemError"],
        test_value["systemError"]
    );
    assert_eq!(
        config.sandbox_status["timeLimitExceeded"],
        test_value["timeLimitExceeded"]
    );
    assert_eq!(program_output_data, expected_program_output_data);
}

#[test]
fn testprogram_python_requests() {
    let uid = std::env::var("SANDBOX_UID").expect("Failed to get uid from env").parse::<u16>().unwrap();
    let gid = std::env::var("SANDBOX_GID").expect("Failed to get gid from env").parse::<u16>().unwrap();
    
    let mut config = sandboxclient::SandboxClient::new();
    config.set_sandbox_executable(&String::from("../build/codemountain_sandbox"));
    config.set_source_file(&String::from("../testprograms/python-requests/program.py"));
    config.set_input_file(&String::from("../testprograms/rm/input.txt"));
    config.set_output_file(&String::from("../build/output.txt"));
    config.set_executable_args(&vec![String::from("../testprograms/python-requests/program.py")]);
    config.set_executable(&String::from("/usr/bin/python"));
    config.set_uid(uid);
    config.set_gid(gid);
    config.set_time_limit(1);
    config.set_memory_limit(128);
    config.gen_cmd();

    // execute
    config.run();

    let mut program_output = match std::fs::File::open("../build/output.txt") {
        Err(why) => panic!("Could not read program output file: {}", why),
        Ok(file) => file,
    };

    let mut program_output_data = String::new();
    match program_output.read_to_string(&mut program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let mut expected_program_output =
        match std::fs::File::open("../testprograms/python-requests/expected_output.txt") {
            Err(why) => panic!("Could not read program output file: {}", why),
            Ok(file) => file,
        };

    let mut expected_program_output_data = String::new();
    match expected_program_output.read_to_string(&mut expected_program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let test_value: serde_json::Value = serde_json::from_str(
        r#"{
            "compileError": 0,
            "memoryLimitExceeded": 0,
            "runtimeError": 0,
            "spentTime": 0,
            "systemError": 0,
            "timeLimitExceeded": 0,
            "usedMemory": 2
          }"#,
    )
    .expect("Error occured while parsing test json data. This shouldn't happen");

    assert_eq!(config.sandbox_failed, false);
    assert_eq!(
        config.sandbox_status["compileError"],
        test_value["compileError"]
    );
    assert_eq!(
        config.sandbox_status["memoryLimitExceeded"],
        test_value["memoryLimitExceeded"]
    );
    assert_eq!(
        config.sandbox_status["runtimeError"],
        test_value["runtimeError"]
    );
    assert_eq!(config.sandbox_status["spentTime"], test_value["spentTime"]);
    assert_eq!(
        config.sandbox_status["systemError"],
        test_value["systemError"]
    );
    assert_eq!(
        config.sandbox_status["timeLimitExceeded"],
        test_value["timeLimitExceeded"]
    );
    assert_eq!(program_output_data, expected_program_output_data);
}

#[test]
fn testprogram_rm() {
    let uid = std::env::var("SANDBOX_UID").expect("Failed to get uid from env").parse::<u16>().unwrap();
    let gid = std::env::var("SANDBOX_GID").expect("Failed to get gid from env").parse::<u16>().unwrap();
    
    let mut config = sandboxclient::SandboxClient::new();
    config.set_sandbox_executable(&String::from("../build/codemountain_sandbox"));
    config.set_source_file(&String::from("../testprograms/rm/rm.c"));
    config.set_input_file(&String::from("../testprograms/rm/input.txt"));
    config.set_output_file(&String::from("../build/output.txt"));
    config.set_compile_cmd(&String::from(
        "/usr/bin/gcc ../testprograms/rm/rm.c -DONLINE_JUDGE -o../build/program",
    ));
    config.set_compiler_output_file(&String::from("../build/compiler_output_file.txt"));
    config.set_executable_args(&Vec::new());
    config.set_executable(&String::from("../build/program"));
    config.set_uid(uid);
    config.set_gid(gid);
    config.set_time_limit(1);
    config.set_memory_limit(128);
    config.gen_cmd();

    // execute
    config.run();

    let mut program_output = match std::fs::File::open("../build/output.txt") {
        Err(why) => panic!("Could not read program output file: {}", why),
        Ok(file) => file,
    };

    let mut program_output_data = String::new();
    match program_output.read_to_string(&mut program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let mut expected_program_output =
        match std::fs::File::open("../testprograms/rm/expected_output.txt") {
            Err(why) => panic!("Could not read program output file: {}", why),
            Ok(file) => file,
        };

    let mut expected_program_output_data = String::new();
    match expected_program_output.read_to_string(&mut expected_program_output_data) {
        Ok(_) => {}
        Err(why) => panic!(
            "Error while trying to read data from program output: {}",
            why
        ),
    }

    let test_value: serde_json::Value = serde_json::from_str(
        r#"{
            "compileError": 0,
            "memoryLimitExceeded": 0,
            "runtimeError": 0,
            "spentTime": 0,
            "systemError": 0,
            "timeLimitExceeded": 0,
            "usedMemory": 2
          }"#,
    )
    .expect("Error occured while parsing test json data. This shouldn't happen");

    assert_eq!(config.sandbox_failed, false);
    assert_eq!(
        config.sandbox_status["compileError"],
        test_value["compileError"]
    );
    assert_eq!(
        config.sandbox_status["memoryLimitExceeded"],
        test_value["memoryLimitExceeded"]
    );
    assert_eq!(
        config.sandbox_status["runtimeError"],
        test_value["runtimeError"]
    );
    assert_eq!(config.sandbox_status["spentTime"], test_value["spentTime"]);
    assert_eq!(
        config.sandbox_status["systemError"],
        test_value["systemError"]
    );
    assert_eq!(
        config.sandbox_status["timeLimitExceeded"],
        test_value["timeLimitExceeded"]
    );
    assert_eq!(program_output_data, expected_program_output_data);
}