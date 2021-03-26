#include <stdio.h>
#include "run.h"

void runner(struct judge_job job) {
    printf("Path to source code: %s\n", job.path_to_source);
    printf("Path to input file: %s\n", job.path_to_input_file);
    printf("Compilation required: %s\n", job.is_compiled ? "Yes": "No");
    job.is_compiled && printf("Compilation command: %s\n", job.compile_command);
    printf("Command to run: %s\n", job.run_command);
    printf("Time limit: %ds\n", job.time_limit);
    printf("Memory limit: %dMB\n", job.memory_limit);
}