#pragma once
#include <string>
#include <fstream>

#define CHILD_FORK_FAILED       "Failed to fork child"
#define RLIMIT_MEM_FAILED       "Failed to set memory limit"
#define EXECVE_FAILED           "Failed to run process with execve"
#define SECCOMP_RULE_FAILED     "Failed to set seccomp rule"
#define SETUID_FAILED           "Failed to set uid"
#define SETGID_FAILED           "Failed to set gid"
#define FILE_ERROR              "Failed to do operations on file"
#define FILE_OPEN_FAILURE       "Failed to open specified file"
#define INVALID_NUMBER          "Expected number but got invalid value"
#define DUP2_FAILED             "Failed to redirect stdout/stdin/stderr to specified file"
#define WAIT_FAILED             "Failed to wait for child process"
#define SETRLIMIT_FAILED        "Failed to set resource limit"

namespace Logger {
    enum LOG_LEVEL
    {
        DEBUG,
        WARNING,
        ERROR,
        FATAL
    };

    class Log
    {
    public:
        Log(std::string);
        ~Log();
        void write_log(LOG_LEVEL, const std::string&);
    private:
        std::fstream m_Log_File; 
    };
}