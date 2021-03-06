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

#include "seccomp_rules.hpp"
#include <seccomp.h>
#include <fcntl.h>
#include <unistd.h>
#include "log.hpp"
#include "config.hpp"

int set_rules(config *sandbox_config)
{
    Logger::Log logger("./logs/SECCOMP-LOG.log");
    scmp_filter_ctx ctx = nullptr;
    ctx = seccomp_init(SCMP_ACT_ALLOW);

    if(!ctx)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        return -1;
    }

    #ifdef DEBUGMODE
        logger.write_log(Logger::LOG_LEVEL::DEBUG, std::string(sandbox_config->binary)+" - Binary file");
    #endif

    // execve
    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(execve), 1, SCMP_A0(SCMP_CMP_NE, (scmp_datum_t)(sandbox_config->binary))) == -1)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        return -1;
    }

    // execveat
    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(execveat), 0) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        return -1;
    }

    // socket
    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(socket), 0) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        return -1;
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
        return -1;
    }

    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(fork), 0) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        return -1;
    }

    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(vfork), 0) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        return -1;
    }

    if(seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(kill), 0) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        return -1;
    }



    if(seccomp_load(ctx) != 0)
    {
        logger.write_log(Logger::LOG_LEVEL::ERROR, std::string(SECCOMP_RULE_FAILED));
        return -1;
    }

    seccomp_release(ctx);

    return 0;
}