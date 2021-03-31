#include "run.hpp"
#include "log.hpp"
#include "compile.hpp"
#include "runtime.hpp"
#include "killer.hpp"
#include <unistd.h>
#include <sys/wait.h>
#include <signal.h>
#include <sys/resource.h>
#include <time.h>

void run(config *sandbox_config, result *result_struct)
{
    Logger::Log logger("./logs/RUNNER-LOG.log");
    if(sandbox_config->compile_command != "")
    {
        // we need to compile the program. no need to seccomp right now
        pid_t compile_pid;
        if((compile_pid = fork()) < 0)
        {
            result_struct->systemError = true;
            logger.write_log(Logger::LOG_LEVEL::ERROR, CHILD_FORK_FAILED);
            return;
        }
        else if(compile_pid == 0)
        {
            compile(sandbox_config, result_struct);
        }
        else if(compile_pid > 0)
        {
            // wait for compile process to die
            int status;
            struct rusage _compile_process_rusage;
            
            if(wait4(compile_pid, &status, WSTOPPED, &_compile_process_rusage) == -1)
            {
                kill(compile_pid, SIGKILL);
                logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(WAIT_FAILED));
                result_struct->systemError = true;
                return;
            }
            if(result_struct->systemError) // no need to continue
            {
                return;
            }
            if(status != 0)
            {
                result_struct->compileErrors = true;
            }
            if(result_struct->compileErrors)
            {
                return;
            }
        }
        pid_t runtime_pid;
        if((runtime_pid = fork()) < 0)
        {
            result_struct->systemError = true;
            logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(CHILD_FORK_FAILED));
            return;
        }
        else if(runtime_pid == 0)
        {
            runtime(sandbox_config, result_struct);
        }
        else if(runtime_pid > 0)
        {
            // wait for runtime process to die
            int status;
            struct rusage runtime_rusage;
            const clock_t begin_time = clock();
            killer k = killer(sandbox_config, result_struct, runtime_pid);

            if(wait4(runtime_pid, &status, WSTOPPED, &runtime_rusage) == -1)
            {
                kill(compile_pid, SIGKILL);
                logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(WAIT_FAILED));
                result_struct->systemError = true;
                return;
            }

            // cancel(will do nothing if already done)
            k.cancel();

            if(result_struct->systemError)
            {
                return; // no need to continue
            }
            
            result_struct->usedMemory = runtime_rusage.ru_maxrss / 1024;
            result_struct->spentTime = (clock() - begin_time) / 1000;

            if(result_struct->timeLimitExceeded)
            {
                return;
            }

            if(result_struct->usedMemory > sandbox_config->memory_limit)
            {
                result_struct->memoryLimitExceeded = true;
            }

            if(WEXITSTATUS(status) != 0)
            {
                if(WEXITSTATUS(status) == SIGUSR1)
                    result_struct->runtimeErrors = true;
                else if(WEXITSTATUS(status) == SIGSEGV)
                {
                    if(result_struct->usedMemory > sandbox_config->memory_limit)
                    {
                        result_struct->memoryLimitExceeded = true;
                    }
                    else
                    {
                        result_struct->runtimeErrors = true;
                    }
                } 
            }
            if(result_struct->spentTime > sandbox_config->time_limit)
            {
                result_struct->timeLimitExceeded = true;
            }
        }
    }
}