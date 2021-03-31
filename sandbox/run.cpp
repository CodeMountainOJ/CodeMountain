#include "run.hpp"
#include "log.hpp"
#include "compile.hpp"
#include <unistd.h>
#include <sys/wait.h>
#include <signal.h>
#include <sys/resource.h>

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
    }
}