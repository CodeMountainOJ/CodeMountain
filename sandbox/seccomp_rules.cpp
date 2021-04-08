#include "seccomp_rules.hpp"
#include <seccomp.h>
#include <fcntl.h>
#include <unistd.h>
#include "log.hpp"

void set_rules(config *sandbox_config, result *result_struct)
{
    Logger::Log logger("./logs/SECCOMP-LOG.log");
    scmp_filter_ctx ctx = nullptr;
    ctx = seccomp_init(SCMP_ACT_ALLOW);

    if(!ctx)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        result_struct->systemError = true;
        return;
    }

    // std::string binary_path = sandbox_config->binary;
    // char* path = new char[binary_path.size() + 1];
    // std::copy(binary_path.begin(), binary_path.end(), path);

    // // execve
    // if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(execve), 1, SCMP_A0(SCMP_CMP_NE, (scmp_datum_t)(path))) == -1)
    // {
    //     logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
    //     result_struct->systemError = true;
    //     return;
    // }

    // execveat
    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(execveat), 0) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        result_struct->systemError = true;
        return;
    }

    // socket
    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(socket), 1, SCMP_A0(SCMP_CMP_NE, (scmp_datum_t)(sandbox_config->binary.c_str()))) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        result_struct->systemError = true;
        return;
    }

    // // no rw, w using open
    // if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(open), 1, SCMP_CMP(1, SCMP_CMP_MASKED_EQ, O_WRONLY, O_WRONLY)) != 0)
    // {
    //     logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
    //     result_struct->systemError = true;
    // }

    // if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(open), 1, SCMP_CMP(1, SCMP_CMP_MASKED_EQ, O_RDWR, O_RDWR)) != 0)
    // {
    //     logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
    //     result_struct->systemError = true;
    // }

    // // no rw, w using openat
    // if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(openat), 1, SCMP_CMP(1, SCMP_CMP_MASKED_EQ, O_WRONLY, O_WRONLY)) != 0)
    // {
    //     logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
    //     result_struct->systemError = true;
    // }

    // if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(openat), 1, SCMP_CMP(1, SCMP_CMP_MASKED_EQ, O_RDWR, O_RDWR)) != 0)
    // {
    //     logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
    //     result_struct->systemError = true;
    // }

    // clone fork vfork kill
    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(clone), 0) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        result_struct->systemError = true;
    }

    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(fork), 0) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        result_struct->systemError = true;
    }

    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(vfork), 0) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        result_struct->systemError = true;
    }

    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(kill), 0) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        result_struct->systemError = true;
    }



    if(seccomp_load(ctx) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        result_struct->systemError = true;
        return;
    }

    seccomp_release(ctx);

    return;
}