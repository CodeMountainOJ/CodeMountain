use clap::{App, Arg};
use cmoj_sandbox_lib::Sandbox;
use shlex::{Shlex, join};

fn parse_int_or_else(i: String) -> Result<(), String> {
  i.parse::<i64>().map_or_else(|_| Err(String::from("Expected a number")), |_| Ok(()))
}

fn main() {
  let app = App::new("CodeMountain Sandbox CLI")
    .about("CLI wrapper for the CMOJ Sandboxing library")
    .arg(Arg::with_name("executable")
                .long("executable")
                .short("e")
                .required(true)
                .takes_value(true)
                .help("Executable to run inside secure environment"))
    .arg(Arg::with_name("realtime-limit")
                .long("realtime-limit")
                .short("r")
                .required(true)
                .validator(parse_int_or_else)
                .takes_value(true)
                .help("Real time limit"))
    .arg(Arg::with_name("cputime-limit")
                .long("cputime-limit")
                .short("c")
                .required(true)
                .validator(parse_int_or_else)
                .takes_value(true)
                .help("CPU time limit"))
    .arg(Arg::with_name("memory-limit")
                .long("memory-limit")
                .short("m")
                .required(true)
                .validator(parse_int_or_else)
                .takes_value(true)
                .help("Memory limit"))
    .arg(Arg::with_name("stack-limit")
                .long("stack-limit")
                .short("s")
                .required(true)
                .validator(parse_int_or_else)
                .takes_value(true)
                .help("Maximum stack for sandboxed process"))
    .arg(Arg::with_name("executable-args")
                .long("executable-args")
                .short("a")
                .required(true)
                .takes_value(true)
                .help("Arugments for running the program (Put in a string)"))
    .arg(Arg::with_name("executable-stdin")
                .long("executable-stdin")
                .short("i")
                .required(true)
                .takes_value(true)
                .help("File to replace the STDIN of the program with(Must exist)"))
    .arg(Arg::with_name("executable-stdout")
                .long("executable-stdout")
                .short("o")
                .required(true)
                .takes_value(true)
                .help("File to store the output of the program in"))
    .arg(Arg::with_name("executable-stderr")
                .long("executable-stderr")
                .short("p")
                .required(true)
                .takes_value(true)
                .help("File to store the STDERR of the program in"));
  
  let matches = app.get_matches();
  
  let executable = matches.value_of("executable").unwrap();
  let realtime_limit = matches.value_of("realtime-limit").unwrap();
  let cputime_limit = matches.value_of("cputime-limit").unwrap();
  let memory_limit = matches.value_of("memory-limit").unwrap();
  let stack_limit = matches.value_of("stack-limit").unwrap();
  let executable_args = {
    let a = matches.value_of("executable-args").unwrap();
    let s = Shlex::new(a);
    s.collect()
  };
  let executable_stdout = matches.value_of("executable-stdout").unwrap();
  let executable_stdin = matches.value_of("executable-stdin").unwrap();
  let executable_stderr = matches.value_of("executable-stderr").unwrap();

  let sandbox = Sandbox::new()
    .set_executable_path(&executable)
    .set_executable_args(&executable_args)
    .set_realtime_limit(realtime_limit.parse::<i64>().unwrap())
    .set_cputime_limit(cputime_limit.parse::<i64>().unwrap())
    .set_memory_limit(memory_limit.parse::<i64>().unwrap())
    .set_stack_limit(stack_limit.parse::<i64>().unwrap())
    .set_executable_stdin(&executable_stdin)
    .set_executable_stdout(&executable_stdout)
    .set_executable_stderr(&executable_stderr);

  let results = sandbox.run();

  println!("{:#?}", results);
}