# Installation

This guide shows how to set up the servers and compile the sandbox for testing or running in local/server.

---
**Warning: None of the servers are ready for what they are actually supposed to do. They are being developed. This manual will be updated later when the development of the first version is finished.**

## Compiling The Sandbox

Compiling the sandbox should be fairly easy. You need to go inside the sandbox directory. Then run execute the ```build.py``` inside that directory. Add ```--debug``` flag to get debug infos in the logs. Where are the logs stored? They are stored in the same directory where the sandbox is running.

### Using It From Shell

Run the sandbox as root and supply --help argument to see available options.

### Using It From A Rust Program

Add the crate inside the ```sandbox/rust-binding``` as your dependency.

Example:

```rust
use sandbox::Sandbox;

fn main() {
    let sb = Sandbox::new(); // make an instance of the Sandbox struct
    
    // set the required values
    sb.set_sandbox_executable(sandbox_executable); // path to compiled sandbox binary
    sb.set_source_file(source_file);
    sb.set_input_file(input_file); // path to input file which will be put into STDIN
    sb.set_output_file(output_file); // path where the program output will be saved
    sb.set_compile_cmd(compile_cmd); // set if the program needs to be compiled
    sb.set_compiler_output_file(compiler_output_file); // set if you set the compile command
    sb.set_executable_args(executable_args);
    sb.set_executable(executable);
    sb.set_uid(uid); // uid to set when the program is running inside the sandbox
    sb.set_gid(gid); // gid ^
    sb.set_time_limit(time_limit); // in seconds
    sb.set_memory_limit(memory_limit); // in megabytes
    sb.gen_cmd();

    // execute the sandbox
    sb.execute();

    // get and print the sandbox status
    println!("{:?}", sb.sandbox_status);
}
```

**Important note: Make sure you run the sandbox, or the rust program running the sandbox as root. Otherwise, it will fail. To prevent that, the sandbox will exit without doing anything. It'll simply write an error inside the log file called "ENTRYPOINT_LOG.log"**

## Getting The API Server Up And Running
This guide will tell you how to test and run the server on your system without installing or having the hassle of setting up various stuff.

### Requirements
- Docker
- Docker Compose

### Testing(No setup needed)
```shell
docker-compose -f docker-compose.test.yml rm && docker-compose -f docker-compose.test.yml build && docker-compose -f docker-compose.test.yml up
```
### Running(Dev environment)
You need to set some environment variables before running the server;

- REDIS_CON: Not used right now. Set anything random
- SMTP_EMAIL: Email to log in to SMTP server
- SMTP_PASSWORD: Password to log in to SMTP server
- SMTP_SERVER: SMTP server address
- JWT_SECRET_KEY: A random secret key for signing JSON web tokens
- DATABASE_URL: Can be a database server you set up, or the server that comes with the dockerfile. To use the server(database is not persistent),
  use this: `postgres://postgres:postgres@postgres:5432/codemountain_dev`
  
Now you can run this to start the server:
```shell
docker-compose -f docker-compose.dev.yml rm && docker-compose -f docker-compose.dev.yml build && docker-compose -f docker-compose.dev.yml up
```
## Note
If you find any issue in this doc, please create an issue in this repo.