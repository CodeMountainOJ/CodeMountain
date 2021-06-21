# Installation

This guide shows how to set up the servers and compile the sandbox for testing or running in local/server.

---
**Warning: None of the servers are ready for what they are actually supposed to do. They are being developed. This manual will be updated later when the development of the first version is done.**

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

**Imporant note: Make sure you run the sandbox or the rust program running the sandbox as root. Otherwise it will fail. To prevent that, the sandbox will exit without doing anything. It'll simply write an error inside the log file called "ENTRYPOINT_LOG.log"**

## Getting The API Server Up And Running

### Requirements

- Latest version of Rust
- Latest version of Postgresql

### Installing all the required stuff

First install and give the postgres user a password. Then create an empty database. If you want to test the server with ```cargo test``` make sure you make a db called ```codemountain_test```. Then, install Diesel Cli using ```cargo install diesel-cli```. Go to ```server/api``` and then run ```diesel migration run```. This will create all the required tables with columns. Make sure you put DATABASE_URL in env. It's gonna be an url like this:

```postgres://username@password:host/db_name```

In case of ```codemountain_test``` db where the username is postgres, password is postgres and the host is 127.0.0.1, the possible url is:

```postgres://postgres@postgres:127.0.0.1/codemountain_test```

After running migrations, insert a dummy user using the following SQL command:

```sql
INSERT INTO users(id, firstname, lastname, username, email, password) VALUES (25, 'John', 'Doe', 'johndoe', 'john_doe@example.com', '$2b$12$dDuxYtY4gfHBrxzZr6d6k.hHI1r9AAOLdTWC1rNSXKULwrpeiZYti')
```

Make sure you are inserting the user into the ```codemountain_test``` database. This will add an user with john_doe@example.com as the email and "password" as the password. Compile the api server using ```cargo build```. To make a release binary, run ```cargo build --release```. Set the environment variables ```DATABASE_URL``` and ```JWT_SECRET_KEY``` in the env or store in a ```.env``` file inside the project directory(```server/api```). Here, ```DATABASE_URL``` should be the connection url for the db and ```JWT_SECRET_KEY``` is the secret key used to sign the access/refreshtokens. Then execute it ```cargo run``` for debug build and ```cargo run --release``` for release build. If you want to run local tests, then run ```cargo test```.
And finally, you should set the REDIS_CON env variable inside the .env or directly in the environment. But, here's a catch. It doesn't do anything right now, so you can set any value into that variable. It'll be relevent in future.

Endpoints of the API are in the server/api/main.rs. API documentation will be created later.
