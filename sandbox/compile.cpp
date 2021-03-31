#include "compile.hpp"
#include <unistd.h>
#include <vector>
#include <iostream>
#include "log.hpp"

std::vector<std::string> space_split(std::string target)
{
    std::string temp;
    std::vector<std::string> result;
    for (auto ch : target)
    {
        if (ch == ' ')
        {
            result.push_back(temp);
            temp = "";
        }
        else
        {
            temp += ch;
        }
    }
    result.push_back(temp);

    return result;
}

void compile(config *sandbox_config, result *result_struct)
{
    Logger::Log logger("./logs/COMPILER-LOG.log");
    std::string compile_command = sandbox_config->compile_command;
    std::vector<char *> arg_v;

    for (auto s : space_split(compile_command))
    {
        char *writable = new char[s.size() + 1];
        std::copy(s.begin(), s.end(), writable);
        arg_v.push_back(const_cast<char *>(writable));
    }

    arg_v.push_back(NULL);

    char **argv = arg_v.data();
    FILE *compiler_output = fopen(sandbox_config->compiler_output_file.c_str(), "w");
    if(dup2(fileno(compiler_output), fileno(stdout)) == -1)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(DUP2_FAILED));
        result_struct->systemError = true;
        exit(1);
    }
    if(dup2(fileno(compiler_output), fileno(stderr)) == -1)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(DUP2_FAILED));
        result_struct->systemError = true;
        exit(1);
    }
    execv(argv[0], &argv[0]);
    logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(EXECVE_FAILED));
    result_struct->systemError = true;
    exit(1);
}