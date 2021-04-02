#pragma once
#include<string>
#include "result.hpp"

struct config {
    std::string input_file;
    std::string output_file; // maybe misleading, but the sandbox program will store the output in the file specified in this string
    std::string source_file;
    std::string compile_command;
    std::string run_command;
    std::string compiler_output_file;
    std::string binary;
    int         memory_limit;
    int         time_limit;
};

void run(config*, result*);