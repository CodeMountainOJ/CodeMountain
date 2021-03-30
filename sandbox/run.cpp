#include "run.hpp"
#include <iostream>

void run(config sandbox_config)
{
    std::cout<<"Source file: "<<sandbox_config.source_file<<std::endl;
    std::cout<<"Input file: "<<sandbox_config.input_file<<std::endl;
    std::cout<<"Output file: "<<sandbox_config.output_file<<std::endl;
    if(sandbox_config.compile_command != "")
        std::cout<<"Compile Command: "<<sandbox_config.compile_command<<std::endl;
    std::cout<<"Run command: "<<sandbox_config.run_command<<std::endl;
    std::cout<<"Time limit: "<<sandbox_config.time_limit<<"s"<<std::endl;
    std::cout<<"Memory limit: "<<sandbox_config.memory_limit<<"MB"<<std::endl;
}