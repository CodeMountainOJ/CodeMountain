#include <iostream>
#include <fstream>
#include "argparse.hpp"
#include "log.hpp"
#include "run.hpp"
#include "result.hpp"
#include <ostream>

int main(int argc, char** argv) {
    const std::string LOG_FILE = "./logs/ENTRYPOINT-LOG.log";
    Logger::Log logger(LOG_FILE);
    argparse::ArgumentParser sandbox_argparse("codemountain_sandbox");
    sandbox_argparse.add_argument("-s", "--source")
                    .help("Source file to compile/run")
                    .required();
    sandbox_argparse.add_argument("-i")
                    .help("Input file to set to stdin of child process")
                    .required();
    sandbox_argparse.add_argument("-c")
                    .help("Command to compile(Do not specify if code doesn't needs to get compiled)");
    sandbox_argparse.add_argument("-g")
                    .help("File to save compiler output")
                    .required();
    sandbox_argparse.add_argument("-o")
                    .help("The file where output will be saved")
                    .required();
    sandbox_argparse.add_argument("-r")
                    .help("Command to use to run the program")
                    .required();
    sandbox_argparse.add_argument("-t")
                    .help("Time limit(in seconds)")
                    .action([&](const std::string &value)
                    {
                        for(char const &c: value)
                        {
                            if(std::isdigit(c) == 0) logger.write_log(Logger::LOG_LEVEL::FATAL, std::string(INVALID_NUMBER));
                        }
                        return std::stoi(value);
                    })
                    .required();
    sandbox_argparse.add_argument("-m")
                    .help("Memory limit(in megabytes)")
                    .action([&](const std::string &value)
                    {
                        for(char const &c: value)
                        {
                            if(std::isdigit(c) == 0) logger.write_log(Logger::LOG_LEVEL::FATAL, std::string(INVALID_NUMBER));
                        }
                        return std::stoi(value);
                    })
                    .required();
                


    try
    {
        sandbox_argparse.parse_args(argc, argv);
        // sanity checks :^)
        {
            std::fstream source_file(sandbox_argparse.get("-s"));
            if(!source_file)
            {
                logger.write_log(Logger::LOG_LEVEL::FATAL, std::string(FILE_OPEN_FAILURE));
            }
        }
        // sanity checks :^)
        {
            std::fstream input_file(sandbox_argparse.get("-i"));
            if(!input_file)
            {
                logger.write_log(Logger::LOG_LEVEL::FATAL, std::string(FILE_OPEN_FAILURE));
            }
        }
        config sandbox_config;
        result sandbox_result;
        sandbox_config.source_file = sandbox_argparse.get("-s");
        sandbox_config.input_file = sandbox_argparse.get("-i");
        if(sandbox_argparse.present("-c").has_value())
            sandbox_config.compile_command = sandbox_argparse.get<std::string>("-c");
        sandbox_config.output_file = sandbox_argparse.get("-o");
        sandbox_config.run_command = sandbox_argparse.get("-r");
        sandbox_config.time_limit = sandbox_argparse.get<int>("-t");
        sandbox_config.memory_limit = sandbox_argparse.get<int>("-m");
        sandbox_config.compiler_output_file = sandbox_argparse.get("-g");
        run(&sandbox_config, &sandbox_result);
        printf("{\n"
               "\t\"compileError\": %d,\n"
               "\t\"runtimeError\": %d,\n"
               "\t\"timeLimitExceeded\": %d,\n"
               "\t\"memoryLimitExceeded\": %d,\n"
               "\t\"systemError\": %d,\n"
               "\t\"usedMemory\": %d,\n"
               "\t\"spentTime\": %d\n"
               "}\n",
               sandbox_result.compileErrors,
               sandbox_result.runtimeErrors,
               sandbox_result.timeLimitExceeded,
               sandbox_result.memoryLimitExceeded,
               sandbox_result.systemError,
               sandbox_result.usedMemory,
               sandbox_result.spentTime
        );
    }
    catch(const std::runtime_error& e)
    {
        std::cout<<e.what()<<std::endl;
        std::cout<<sandbox_argparse;
        exit(1);
    }
}