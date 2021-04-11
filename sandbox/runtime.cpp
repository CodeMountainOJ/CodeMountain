#include "runtime.hpp"
#include "utils.hpp"
#include "log.hpp"
#include <unistd.h>
#include "seccomp_rules.hpp"
#include <sys/resource.h>
#include <iostream>

void runtime(config* sandbox_config, result* result_struct)
{
    FILE *runtime_input = fopen(sandbox_config->input_file.c_str(), "r");
    FILE *runtime_output = fopen(sandbox_config->output_file.c_str(), "w");
    Logger::Log logger("./logs/RUNTIME-LOG.log");
#ifdef DEBUGMODE
    logger.write_log(Logger::LOG_LEVEL::DEBUG, sandbox_config->runtime_argv + " - These arguments will provided to the program");
    logger.write_log(Logger::LOG_LEVEL::DEBUG, std::string(sandbox_config->binary) + " - Binary program");
#endif

    std::vector<std::string> splitted_command;
    splitted_command.push_back(sandbox_config->binary);
    for(auto arg: space_split(sandbox_config->runtime_argv))
    {
#ifdef DEBUGMODE
        logger.write_log(Logger::LOG_LEVEL::DEBUG, arg + " - arg");
#endif
        splitted_command.push_back(arg);
    }

    std::vector<char*> arg_v;

    for(auto s: splitted_command)
    {
        char* writable = new char[s.size() + 1];
        std::copy(s.begin(), s.end(), writable);
#ifdef DEBUGMODE
        logger.write_log(Logger::LOG_LEVEL::DEBUG, s + " - writable format of argument");
#endif
        arg_v.push_back(writable);
    }
    arg_v.push_back(NULL);
    char **argv = arg_v.data();

    if(runtime_input == NULL)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(FILE_OPEN_FAILURE));
        result_struct->systemError = true;
        exit(1);
    }

    if(runtime_output == NULL)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(FILE_OPEN_FAILURE));
        result_struct->systemError = true;
        exit(1);
    }

    if(dup2(fileno(runtime_output), fileno(stdout)) == -1)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(DUP2_FAILED));
        result_struct->systemError = true;
        exit(1);
    }

    if(dup2(fileno(runtime_output), fileno(stderr)) == -1)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(DUP2_FAILED));
        result_struct->systemError = true;
        exit(1);
    }

    if(dup2(fileno(runtime_input), fileno(stdin)) == -1)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(DUP2_FAILED));
        result_struct->systemError = true;
        exit(1);
    }
    
    // setrlimit
    struct rlimit max_mem;
    max_mem.rlim_cur = max_mem.rlim_max = sandbox_config->memory_limit * 1024 * 1024 * 2;
    if(setrlimit(RLIMIT_AS, &max_mem) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(RLIMIT_MEM_FAILED));
        result_struct->systemError = true;
        exit(1);
    }


    set_rules(sandbox_config, result_struct); // seccomp
    if(result_struct->systemError) // failed to set seccomp rule
    {
        exit(1);
    }

    execve(sandbox_config->binary, &argv[0], environ);
    logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(EXECVE_FAILED));
    result_struct->systemError = true;
    exit(1);
}