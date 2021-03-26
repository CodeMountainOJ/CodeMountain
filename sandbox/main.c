#include <stdio.h>
#include "argparse.h"
#include "run.h"

static const char *const usage[] = {
    "codemountain_sandbox [options]",
    NULL,
};

int
main(int argc, const char **argv)
{
    const char *path_to_source = NULL;
    const char *compile_command = NULL;
    const char *run_command = NULL;
    const char *path_to_input_file = NULL;
    int is_compiled = 0;
    int memory_limit = 0;
    int time_limit = 0;

    struct argparse_option options[] = {
        OPT_HELP(),
        OPT_GROUP("Required options"),
        OPT_STRING('s', "source", &path_to_source, "Path to source code"),
        OPT_STRING('c', "compile-command", &compile_command, "Required if 'compiled' or 'o' flag is set. This command is used to compiled the source code."),
        OPT_STRING('r', "run-command", &run_command, "Command used to run the program"),
        OPT_STRING('i', "input-file", &path_to_input_file, "Path to input file to write to STDIN"),
        OPT_BOOLEAN('o', "compiled", &is_compiled, "Set if source file needs to be compiled"),
        OPT_INTEGER('t', "time-limit", &time_limit, "Maximum time allowed for program to run"),
        OPT_INTEGER('m', "memory-limit", &memory_limit, "Maximum memory allowed for the program"),
        OPT_END()
    };
    
    struct argparse argparse;
    argparse_init(&argparse, options, usage, 0);
    argparse_describe(&argparse, "\nThe code sandboxing tool for CodeMountain.", "\nRuns program inside a secure environment with time and memory limitations");
    argc = argparse_parse(&argparse, argc, argv);

    if(is_compiled) {
        if(!compile_command) {
            printf("\"compile_command\" argument is required if \"is_compiled\" flag is set\n");
            argparse_usage(&argparse);
            return 1;
        }
    }

    if(!path_to_source) {
        printf("A source code file must be specified!\n");
        argparse_usage(&argparse);
        return 1;
    }
    if(!run_command) {
        printf("I don't know how to run the program. Please inform me by specifying the run command using the \"run-command\" flag\n");
        argparse_usage(&argparse);
        return 1;
    }
    if(!path_to_input_file) {
        printf("An input file must be specified!\n");
        argparse_usage(&argparse);
        return 1;
    }
    if(!memory_limit) {
        printf("Memory limit is not specified!\n");
        argparse_usage(&argparse);
        return 1;
    }
    if(!time_limit) {
        printf("Time limit is not specifed!\n");
        argparse_usage(&argparse);
        return 1;
    }

    struct judge_job job;
    job.path_to_source = path_to_source;
    job.path_to_input_file = path_to_input_file;
    job.is_compiled = is_compiled;
    job.compile_command = compile_command;
    job.run_command = run_command;
    job.memory_limit = memory_limit;
    job.time_limit = time_limit;

    runner(job);
}