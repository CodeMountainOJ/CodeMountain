#include "runtime.hpp"
#include "utils.hpp"
#include "log.hpp"
#include <unistd.h>
#include "seccomp_rules.hpp"
#include <sys/resource.h>

void runtime(config* sandbox_config, result* result_struct)
{
    Logger::Log logger("./logs/RUNTIME-LOG.log");
    std::vector<std::string> splitted_command = space_split(sandbox_config->run_command);
    std::vector<char*> arg_v;

    for(auto s: splitted_command)
    {
        char* writable = new char[s.size() + 1];
        std::copy(s.begin(), s.end(), writable);
        arg_v.push_back(writable);
    }
    arg_v.push_back(NULL);

    char **argv = arg_v.data();
    FILE *runtime_output = fopen(sandbox_config->output_file.c_str(), "w");
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
    
    // setrlimit
    struct rlimit max_mem;
    max_mem.rlim_cur = max_mem.rlim_max = sandbox_config->memory_limit * 1024 * 1024 * 2;
    if(setrlimit(RLIMIT_AS, &max_mem) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(RLIMIT_MEM_FAILED));
        result_struct->systemError = true;
        exit(1);
    }


    set_rules(); // seccomp

    execv(argv[0], &argv[0]);
    logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(EXECVE_FAILED));
    result_struct->systemError = true;
    exit(1);
}