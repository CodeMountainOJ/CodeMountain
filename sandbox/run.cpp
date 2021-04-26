/* 
 *  CodeMountain is a free and open source online judge open for everyone
 *  Copyright (C) 2021 MD Gaziur Rahman Noor and contributors
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

#include "run.hpp"
#include "log.hpp"
#include "compile.hpp"
#include "runtime.hpp"
#include "killer.hpp"
#include <unistd.h>
#include <sys/wait.h>
#include <signal.h>
#include <sys/resource.h>
#include <sys/time.h>
#include <iostream>
#include <chrono>

void run(config *sandbox_config, result *result_struct)
{
    Logger::Log logger("./logs/RUNNER-LOG.log");
    if (sandbox_config->compile_command != "")
    {
        // we need to compile the program. no need to seccomp right now
        pid_t compile_pid;
        if ((compile_pid = fork()) < 0)
        {
            result_struct->systemError = true;
            logger.write_log(Logger::LOG_LEVEL::ERROR, CHILD_FORK_FAILED);
            return;
        }
        else if (compile_pid == 0)
        {
            compile(sandbox_config, result_struct);
        }
        else if (compile_pid > 0)
        {
            // wait for compile process to die
            int status;
            struct rusage _compile_process_rusage;

            if (wait4(compile_pid, &status, WSTOPPED, &_compile_process_rusage) == -1)
            {
                kill(compile_pid, SIGKILL);
                logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(WAIT_FAILED));
                result_struct->systemError = true;
                return;
            }

            if (WTERMSIG(status) == SIGUSR1) // no need to continue
            {
                result_struct->systemError = true;
                return;
            }

            if (status != 0)
            {
                result_struct->compileErrors = true;
            }
            if (result_struct->compileErrors)
            {
                return;
            }
        }
    }

    pid_t runtime_pid;
    if ((runtime_pid = fork()) < 0)
    {
        result_struct->systemError = true;
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(CHILD_FORK_FAILED));
        return;
    }
    else if (runtime_pid == 0)
    {
        runtime(sandbox_config, result_struct);
    }
    else if (runtime_pid > 0)
    {
        // wait for runtime process to die
        int status;
        struct rusage runtime_rusage;
        struct timeval start, end;
        gettimeofday(&start, NULL);
        killer k = killer(sandbox_config, result_struct, runtime_pid);

        if (wait4(runtime_pid, &status, WSTOPPED, &runtime_rusage) == -1)
        {
            kill(runtime_pid, SIGKILL);
            logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(WAIT_FAILED));
            result_struct->systemError = true;
            return;
        }

        // cancel(will do nothing if already done)
        k.cancel();

#ifdef DEBUGMODE
        logger.write_log(Logger::LOG_LEVEL::DEBUG, std::string("Sandboxed process's return code - ") + std::to_string(WEXITSTATUS(status)));
#endif

        if (WTERMSIG(status) == SIGUSR1) // no need to continue
        {
            result_struct->systemError = true;
            return;
        }

        result_struct->usedMemory = runtime_rusage.ru_maxrss / 1024;
        gettimeofday(&end, NULL);
        result_struct->spentTime = (int)(end.tv_sec * 1000 + end.tv_usec / 1000 - start.tv_sec * 1000 - start.tv_usec / 1000) / 1000;

        if (result_struct->timeLimitExceeded)
        {
            return;
        }

        if (result_struct->usedMemory > sandbox_config->memory_limit)
        {
            result_struct->memoryLimitExceeded = true;
        }

        if (WEXITSTATUS(status) != 0)
        {
            if (WEXITSTATUS(status) == ENOMEM)
            {
                result_struct->memoryLimitExceeded = true;
            }
            else if (WEXITSTATUS(status) == SIGSEGV)
            {
                if (result_struct->usedMemory > sandbox_config->memory_limit)
                {
                    result_struct->memoryLimitExceeded = true;
                }
                else
                {
                    result_struct->runtimeErrors = true;
                }
            }
            else
                result_struct->runtimeErrors = true;
        }

        if (result_struct->spentTime > sandbox_config->time_limit)
        {
            result_struct->timeLimitExceeded = true;
        }
    }
}