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
use sandbox::sandbox;
use std::io::Read;

struct SandboxOutputData {
    pub compile_error: u8,
    pub memory_limit_exceeded: u8,
    pub runtime_error: u8,
    pub time_limit_exceeded: u8,
    pub system_error: u8,
    pub runtime_error_signal: u8
}

fn testfunction_wc(
    sandbox_executable: &String,
    source_file: &String,
    input_file: &String,
    output_file: &String,
    compile_cmd: &String,
    compiler_output_file: &String,
    executable_args: &Vec<String>,
    executable: &String,
    time_limit: u16,
    memory_limit: u16,
    expected_program_output_data_file: &String,
    testdata: &SandboxOutputData,
) {
    let uid = std::env::var("SANDBOX_UID")
        .expect("Failed to get uid from env")
        .parse::<u16>()
        .unwrap();
    let gid = std::env::var("SANDBOX_GID")
        .expect("Failed to get gid from env")
        .parse::<u16>()
        .unwrap();

    let mut config = sandbox::Sandbox::new();
    config.set_sandbox_executable(sandbox_executable);
    config.set_source_file(source_file);
    config.set_input_file(input_file);
    config.set_output_file(output_file);
    config.set_compile_cmd(compile_cmd);
    config.set_compiler_output_file(compiler_output_file);
    config.set_executable_args(executable_args);
    config.set_executable(executable);
    config.set_uid(uid);
    config.set_gid(gid);
    config.set_time_limit(time_limit);
    config.set_memory_limit(memory_limit);
    config.gen_cmd();

    // execute
    config.run();

    let mut program_output = match std::fs::File::open(output_file) {
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

    let mut expected_program_output = match std::fs::File::open(expected_program_output_data_file) {
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

    assert_eq!(config.sandbox_failed, false);
    assert_eq!(
        config.sandbox_status["compileError"],
        testdata.compile_error
    );
    assert_eq!(
        config.sandbox_status["memoryLimitExceeded"],
        testdata.memory_limit_exceeded
    );
    assert_eq!(
        config.sandbox_status["runtimeError"],
        testdata.runtime_error
    );
    assert_eq!(config.sandbox_status["systemError"], testdata.system_error);
    assert_eq!(
        config.sandbox_status["timeLimitExceeded"],
        testdata.time_limit_exceeded
    );
    assert_eq!(
        config.sandbox_status["runtimeErrorSignal"],
        testdata.runtime_error_signal
    );
    assert_eq!(program_output_data, expected_program_output_data);
}

fn testfunction(
    sandbox_executable: &String,
    source_file: &String,
    input_file: &String,
    output_file: &String,
    executable_args: &Vec<String>,
    executable: &String,
    time_limit: u16,
    memory_limit: u16,
    expected_program_output_data_file: &String,
    testdata: &SandboxOutputData,
) {
    let uid = std::env::var("SANDBOX_UID")
        .expect("Failed to get uid from env")
        .parse::<u16>()
        .unwrap();
    let gid = std::env::var("SANDBOX_GID")
        .expect("Failed to get gid from env")
        .parse::<u16>()
        .unwrap();

    let mut config = sandbox::Sandbox::new();
    config.set_sandbox_executable(sandbox_executable);
    config.set_source_file(source_file);
    config.set_input_file(input_file);
    config.set_output_file(output_file);
    config.set_executable_args(executable_args);
    config.set_executable(executable);
    config.set_uid(uid);
    config.set_gid(gid);
    config.set_time_limit(time_limit);
    config.set_memory_limit(memory_limit);
    config.gen_cmd();

    // execute
    config.run();

    let mut program_output = match std::fs::File::open(output_file) {
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

    let mut expected_program_output = match std::fs::File::open(expected_program_output_data_file) {
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

    assert_eq!(config.sandbox_failed, false);
    assert_eq!(
        config.sandbox_status["compileError"],
        testdata.compile_error
    );
    assert_eq!(
        config.sandbox_status["memoryLimitExceeded"],
        testdata.memory_limit_exceeded
    );
    assert_eq!(
        config.sandbox_status["runtimeError"],
        testdata.runtime_error
    );
    assert_eq!(config.sandbox_status["systemError"], testdata.system_error);
    assert_eq!(
        config.sandbox_status["timeLimitExceeded"],
        testdata.time_limit_exceeded
    );
    assert_eq!(
        config.sandbox_status["runtimeErrorSignal"],
        testdata.runtime_error_signal
    );
    assert_eq!(program_output_data, expected_program_output_data);
}

#[test]
fn testprogram_execve() {
    testfunction_wc(
        &String::from("../build/codemountain_sandbox"),
        &String::from("../testprograms/execve/execve.c"),
        &String::from("../testprograms/execve/input.txt"),
        &String::from("../testprograms/execve/output.txt"),
        &String::from("/bin/gcc ../testprograms/execve/execve.c -o../build/program -DONLINE_JUDGE"),
        &String::from("../build/compiler_output.txt"),
        &Vec::new(),
        &String::from("../build/program"),
        1,
        128,
        &String::from("../testprograms/execve/output.txt"),
        &SandboxOutputData {
            compile_error: 0,
            memory_limit_exceeded: 0,
            runtime_error: 1,
            system_error: 0,
            time_limit_exceeded: 0,
            runtime_error_signal: 31
        },
    );
}

#[test]
fn testprogram_fs() {
    testfunction_wc(
        &String::from("../build/codemountain_sandbox"),
        &String::from("../testprograms/fs/fs.c"),
        &String::from("../testprograms/fs/input.txt"),
        &String::from("../testprograms/fs/output.txt"),
        &String::from("/bin/gcc ../testprograms/fs/fs.c -o../build/program -DONLINE_JUDGE"),
        &String::from("../build/compiler_output.txt"),
        &Vec::new(),
        &String::from("../build/program"),
        1,
        128,
        &String::from("../testprograms/fs/output.txt"),
        &SandboxOutputData {
            compile_error: 0,
            memory_limit_exceeded: 0,
            runtime_error: 0,
            system_error: 0,
            time_limit_exceeded: 0,
            runtime_error_signal: 0
        },
    );
}

#[test]
fn testprogram_io() {
    testfunction_wc(
        &String::from("../build/codemountain_sandbox"),
        &String::from("../testprograms/io/program.c"),
        &String::from("../testprograms/io/input.txt"),
        &String::from("../testprograms/io/output.txt"),
        &String::from("/bin/gcc ../testprograms/io/program.c -o../build/program -DONLINE_JUDGE"),
        &String::from("../build/compiler_output.txt"),
        &Vec::new(),
        &String::from("../build/program"),
        1,
        128,
        &String::from("../testprograms/io/output.txt"),
        &SandboxOutputData {
            compile_error: 0,
            memory_limit_exceeded: 0,
            runtime_error: 0,
            system_error: 0,
            time_limit_exceeded: 0,
            runtime_error_signal: 0
        },
    );
}

#[test]
fn testprogram_python_helloworld() {
    testfunction(
        &String::from("../build/codemountain_sandbox"),
        &String::from("../testprograms/python-helloworld/program.py"),
        &String::from("../testprograms/python-helloworld/input.txt"),
        &String::from("../build/output.txt"),
        &vec![String::from("../testprograms/python-helloworld/program.py")],
        &String::from("/usr/bin/python"),
        1,
        128,
        &String::from("../testprograms/python-helloworld/output.txt"),
        &SandboxOutputData {
            compile_error: 0,
            memory_limit_exceeded: 0,
            runtime_error: 0,
            system_error: 0,
            time_limit_exceeded: 0,
            runtime_error_signal: 0
        },
    )
}

#[test]
fn testprogram_python_input() {
    testfunction(
        &String::from("../build/codemountain_sandbox"),
        &String::from("../testprograms/python-input/program.py"),
        &String::from("../testprograms/python-input/input.txt"),
        &String::from("../build/output.txt"),
        &vec![String::from("../testprograms/python-input/program.py")],
        &String::from("/usr/bin/python"),
        1,
        128,
        &String::from("../testprograms/python-input/output.txt"),
        &SandboxOutputData {
            compile_error: 0,
            memory_limit_exceeded: 0,
            runtime_error: 0,
            system_error: 0,
            time_limit_exceeded: 0,
            runtime_error_signal: 0 
        },
    )
}

#[test]
fn testprogram_python_requests() {
    testfunction(
        &String::from("../build/codemountain_sandbox"),
        &String::from("../testprograms/python-requests/program.py"),
        &String::from("../testprograms/python-requests/input.txt"),
        &String::from("../build/output.txt"),
        &vec![String::from("../testprograms/python-requests/program.py")],
        &String::from("/usr/bin/python"),
        1,
        128,
        &String::from("../testprograms/python-requests/output.txt"),
        &SandboxOutputData {
            compile_error: 0,
            memory_limit_exceeded: 0,
            runtime_error: 1,
            system_error: 0,
            time_limit_exceeded: 0,
            runtime_error_signal: 31
        },
    )
}

#[test]
fn testprogram_rm() {
    testfunction_wc(
        &String::from("../build/codemountain_sandbox"),
        &String::from("../testprograms/rm/rm.c"),
        &String::from("../testprograms/rm/input.txt"),
        &String::from("../testprograms/rm/output.txt"),
        &String::from("/bin/gcc ../testprograms/rm/rm.c -o../build/program -DONLINE_JUDGE"),
        &String::from("../build/compiler_output.txt"),
        &Vec::new(),
        &String::from("../build/program"),
        1,
        128,
        &String::from("../testprograms/rm/output.txt"),
        &SandboxOutputData {
            compile_error: 0,
            memory_limit_exceeded: 0,
            runtime_error: 1,
            system_error: 0,
            time_limit_exceeded: 0,
            runtime_error_signal: 31
        },
    );
}

#[test]
fn testprogram_segfault() {
    testfunction_wc(
        &String::from("../build/codemountain_sandbox"),
        &String::from("../testprograms/segfault/segfault.c"),
        &String::from("../testprograms/segfault/input.txt"),
        &String::from("../testprograms/segfault/output.txt"),
        &String::from(
            "/bin/gcc ../testprograms/segfault/segfault.c -o../build/program -DONLINE_JUDGE",
        ),
        &String::from("../build/compiler_output.txt"),
        &Vec::new(),
        &String::from("../build/program"),
        1,
        128,
        &String::from("../testprograms/segfault/output.txt"),
        &SandboxOutputData {
            compile_error: 0,
            memory_limit_exceeded: 0,
            runtime_error: 1,
            system_error: 0,
            time_limit_exceeded: 0,
            runtime_error_signal: 11
        },
    );
}
