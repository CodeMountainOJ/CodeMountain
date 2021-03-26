struct judge_job{
    const char *path_to_source;
    const char *compile_command;
    const char *run_command;
    const char *path_to_input_file;
    int is_compiled;
    int memory_limit;
    int time_limit;
};

void runner(struct judge_job);