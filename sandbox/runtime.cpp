/* 
 *  CodeMountain is a free and open source online judge open for everyone
 *  Copyright (C) 2021 MD Gaziur Rahman Noor
 *  
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *  
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *  
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#include "runtime.hpp"
#include "utils.hpp"
#include "log.hpp"
#include <unistd.h>
#include "seccomp_rules.hpp"
#include <sys/resource.h>
#include <iostream>
#include <errno.h>
#include "signal.hpp"

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
        writable[s.size()] = '\0';
        arg_v.push_back(writable);
    }
    arg_v.push_back(NULL);
    char **argv = arg_v.data();

    if(runtime_input == NULL)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(FILE_OPEN_FAILURE));
        result_struct->systemError = true;
        systemError();
    }

    if(runtime_output == NULL)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(FILE_OPEN_FAILURE));
        result_struct->systemError = true;
        systemError();
    }

    if(dup2(fileno(runtime_output), fileno(stdout)) == -1)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(DUP2_FAILED));
        result_struct->systemError = true;
        systemError();
    }

    if(dup2(fileno(runtime_output), fileno(stderr)) == -1)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(DUP2_FAILED));
        result_struct->systemError = true;
        systemError();
    }

    if(dup2(fileno(runtime_input), fileno(stdin)) == -1)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(DUP2_FAILED));
        result_struct->systemError = true;
        systemError();
    }
    
    // setrlimit
    struct rlimit max_mem;
    max_mem.rlim_cur = max_mem.rlim_max = sandbox_config->memory_limit * 1024 * 1024 * 2;
    if(setrlimit(RLIMIT_AS, &max_mem) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(RLIMIT_MEM_FAILED));
        result_struct->systemError = true;
        systemError();
    }


    set_rules(sandbox_config, result_struct); // seccomp
    if(result_struct->systemError) // failed to set seccomp rule
    {
        systemError();
    }

    if(setgid(sandbox_config->child_gid) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SETGID_FAILED));
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string("Errno: ") + std::to_string(errno));
        result_struct->systemError = true;
        perror("setgid");
        systemError();
    }
#ifdef DEBUGMODE
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string("Current euid: ") + std::to_string(geteuid()));
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string("Current uid: ") + std::to_string(getuid()));
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string("Current egid: ") + std::to_string(getegid()));
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string("Current gid: ") + std::to_string(getgid()));
#endif

    if(setuid(sandbox_config->child_uid) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SETUID_FAILED));
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string("Errno: ") + std::to_string(errno));
        result_struct->systemError = true;
        perror("setgid");
        systemError();
    }
#ifdef DEBUGMODE
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string("Current euid: ") + std::to_string(geteuid()));
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string("Current uid: ") + std::to_string(getuid()));
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string("Current egid: ") + std::to_string(getegid()));
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string("Current gid: ") + std::to_string(getgid()));
#endif

    execve(sandbox_config->binary, &argv[0], environ);
    logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(EXECVE_FAILED));
    result_struct->systemError = true;
    systemError();
}