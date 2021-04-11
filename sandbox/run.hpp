#pragma once
#include<string>
#include "result.hpp"

struct config {
    std::string input_file;
    std::string output_file; // maybe misleading, but the sandbox program will store the output in the file specified in this string
    std::string source_file;
    std::string compile_command;
    std::string runtime_argv;
    std::string compiler_output_file;
    char*       binary;
    int         memory_limit;
    int         time_limit;
};

void run(config*, result*);