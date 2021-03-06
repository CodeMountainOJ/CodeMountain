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

#[test]
fn init_test() {
    let config = sandbox::Sandbox::new();

    assert_eq!(config.sandbox_executable, String::from(""));
    assert_eq!(config.source_file, String::from(""));
    assert_eq!(config.input_file, String::from(""));
    assert_eq!(config.output_file, String::from(""));
    assert_eq!(config.compiler_output_file, String::from(""));
    assert_eq!(config.compile_cmd, String::from(""));
    assert_eq!(config.executable, String::from(""));
    assert_eq!(config.executable_args.len(), 0);
    assert_eq!(config.uid, 0);
    assert_eq!(config.gid, 0);
    assert_eq!(config.time_limit, 0);
    assert_eq!(config.memory_limit, 0);
    assert_eq!(config.initialized, false);
}

#[test]
fn setting_values() {
    let mut config = sandbox::Sandbox::new();
    let generated_cmd = vec![
        "-s",
        "../testprograms/rm/rm.c",
        "-i",
        "../testprograms/rm/input.txt",
        "-o",
        "../build/output.txt",
        "-c",
        "/usr/bin/gcc rm.c -DONLINE_JUDGE -oprogram",
        "-g",
        "../build/compiler_output_file.txt",
        "-e",
        "program",
        "-r",
        "",
        "-t",
        "1",
        "-m",
        "128",
        "-u",
        "1001",
        "-d",
        "1001",
    ];

    config.set_sandbox_executable(&String::from("../build/codemountain_sandbox"));
    config.set_source_file(&String::from("../testprograms/rm/rm.c"));
    config.set_input_file(&String::from("../testprograms/rm/input.txt"));
    config.set_output_file(&String::from("../build/output.txt"));
    config.set_compile_cmd(&String::from("/usr/bin/gcc rm.c -DONLINE_JUDGE -oprogram"));
    config.set_compiler_output_file(&String::from("../build/compiler_output_file.txt"));
    config.set_executable_args(&Vec::new());
    config.set_executable(&String::from("program"));
    config.set_uid(1001);
    config.set_gid(1001);
    config.set_time_limit(1);
    config.set_memory_limit(128);
    config.gen_cmd();

    assert_eq!(config.get_cmd(), generated_cmd);
}

#[test]
fn setting_values_with_args_filled() {
    let args: Vec<String> = vec![String::from("-a"), String::from("-b")];
    let mut config = sandbox::Sandbox::new();
    let generated_cmd = vec![
        "-s",
        "../testprograms/rm/rm.c",
        "-i",
        "../testprograms/rm/input.txt",
        "-o",
        "../build/output.txt",
        "-c",
        "/usr/bin/gcc rm.c -DONLINE_JUDGE -oprogram",
        "-g",
        "../build/compiler_output_file.txt",
        "-e",
        "program",
        "-r",
        "-a -b",
        "-t",
        "1",
        "-m",
        "128",
        "-u",
        "1001",
        "-d",
        "1001",
    ];

    config.set_sandbox_executable(&String::from("../build/codemountain_sandbox"));
    config.set_source_file(&String::from("../testprograms/rm/rm.c"));
    config.set_input_file(&String::from("../testprograms/rm/input.txt"));
    config.set_output_file(&String::from("../build/output.txt"));
    config.set_compile_cmd(&String::from("/usr/bin/gcc rm.c -DONLINE_JUDGE -oprogram"));
    config.set_compiler_output_file(&String::from("../build/compiler_output_file.txt"));
    config.set_executable_args(&args);
    config.set_executable(&String::from("program"));
    config.set_uid(1001);
    config.set_gid(1001);
    config.set_time_limit(1);
    config.set_memory_limit(128);
    config.gen_cmd();

    assert_eq!(config.get_cmd(), generated_cmd);
}

#[test]
fn setting_values_but_no_compilation() {
    let mut config = sandbox::Sandbox::new();
    let generated_cmd = vec![
        "-s",
        "../testprograms/python-helloworld/program.py",
        "-i",
        "../testprograms/python-helloworld/input.txt",
        "-o",
        "../build/output.txt",
        "-e",
        "/usr/bin/python",
        "-r",
        "../testprograms/python-helloworld/program.py",
        "-t",
        "1",
        "-m",
        "128",
        "-u",
        "1001",
        "-d",
        "1001",
    ];

    let args = vec![String::from("../testprograms/python-helloworld/program.py")];
    config.set_sandbox_executable(&String::from("../build/codemountain_sandbox"));
    config.set_source_file(&String::from(
        "../testprograms/python-helloworld/program.py",
    ));
    config.set_input_file(&String::from("../testprograms/python-helloworld/input.txt"));
    config.set_output_file(&String::from("../build/output.txt"));
    config.set_executable_args(&args);
    config.set_executable(&String::from("/usr/bin/python"));
    config.set_uid(1001);
    config.set_gid(1001);
    config.set_time_limit(1);
    config.set_memory_limit(128);
    config.gen_cmd();

    assert_eq!(config.get_cmd(), generated_cmd);
}

#[test]
fn uninitialized_test() {
    let mut config = sandbox::Sandbox::new();
    config.set_sandbox_executable(&String::from("../build/codemountain_sandbox"));
    config.check_is_initialized();
    assert_eq!(config.initialized, false);
}