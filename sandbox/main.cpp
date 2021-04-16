#include <iostream>
#include <fstream>
#include "argparse.hpp"
#include "log.hpp"
#include "run.hpp"
#include "result.hpp"
#include <ostream>
#include <unistd.h>

int main(int argc, char** argv) {
    const std::string LOG_FILE = "./logs/ENTRYPOINT-LOG.log";
    Logger::Log logger(LOG_FILE);
    // immediately check if the program is started as root(this will allow us to setuid to any uid)
    if(getuid() != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::FATAL, std::string(NOT_ROOT));
    }

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
                    .help("File to save compiler output");
    sandbox_argparse.add_argument("-o")
                    .help("The file where output will be saved")
                    .required();
    sandbox_argparse.add_argument("-r")
                    .help("Arguments to use when running the program")
                    .required();
    sandbox_argparse.add_argument("-e")
                    .help("Binary program to run")
                    .required();
    sandbox_argparse.add_argument("-u")
                    .help("UID for the sandboxed program")
                    .action([&](const std::string &value)
                    {
                        for(char const &c: value)
                        {
                            if(std::isdigit(c) == 0) logger.write_log(Logger::LOG_LEVEL::FATAL, std::string(INVALID_NUMBER));
                        }
                        return std::stoi(value);
                    })
                    .required();
    sandbox_argparse.add_argument("-d")
                    .help("GID for the sandboxed program")
                    .action([&](const std::string &value)
                    {
                        for(char const &c: value)
                        {
                            if(std::isdigit(c) == 0) logger.write_log(Logger::LOG_LEVEL::FATAL, std::string(INVALID_NUMBER));
                        }
                        return std::stoi(value);
                    })
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
        sandbox_config.source_file = sandbox_argparse.get<std::string>("-s");
        sandbox_config.input_file = sandbox_argparse.get<std::string>("-i");
        if(sandbox_argparse.present("-c").has_value())
        {
            if(!sandbox_argparse.present("-g").has_value())
            {
                std::cout<<"Expected -g because -c is provided"<<std::endl;
                std::cout<<sandbox_argparse;
                exit(0);
            }
            sandbox_config.compiler_output_file = sandbox_argparse.get<std::string>("-g");
            sandbox_config.compile_command = sandbox_argparse.get<std::string>("-c");
#ifdef DEBUGMODE
            logger.write_log(Logger::LOG_LEVEL::DEBUG, sandbox_config.compiler_output_file + " - Compiler output file" );
            logger.write_log(Logger::LOG_LEVEL::DEBUG, sandbox_config.compile_command + " - Compiler command" );
#endif
        }
        sandbox_config.output_file = sandbox_argparse.get<std::string>("-o");
        sandbox_config.runtime_argv = sandbox_argparse.get<std::string>("-r");
        sandbox_config.time_limit = sandbox_argparse.get<int>("-t");
        sandbox_config.memory_limit = sandbox_argparse.get<int>("-m");
        sandbox_config.runtime_argv = sandbox_argparse.get<std::string>("-r");
        sandbox_config.child_uid = sandbox_argparse.get<int>("-u");
        sandbox_config.child_gid = sandbox_argparse.get<int>("-d");

        std::string binary = sandbox_argparse.get<std::string>("-e");
        char* writable = new char[binary.size() + 1];
        std::copy(binary.begin(), binary.end(), writable);
        writable[binary.size()] = '\0';
        sandbox_config.binary = writable;
#ifdef DEBUGMODE
        logger.write_log(Logger::LOG_LEVEL::DEBUG, std::string(writable) + " - Binary file");
        logger.write_log(Logger::LOG_LEVEL::DEBUG, std::string(std::to_string(binary.size())) + " - Binary filename size");
#endif

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
        exit(0);
    }
}