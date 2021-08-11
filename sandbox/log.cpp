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

#include "log.hpp"
#include <iostream>
#include <vector>
#include <ctime>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#include <filesystem>

Logger::Log::Log(std::string log_file_name)
{
    if(std::filesystem::exists(log_file_name)) {
        m_Log_File = std::fstream(log_file_name, std::ios::app);
        if(chmod(log_file_name.c_str(), S_IRWXU | S_IRWXG | S_IRWXO) != 0) {
            std::cout<<"[FATAL] Cannot chmod() on log files!"<<std::endl;
            perror("chmod");
        }
    } else {
        mkdir("logs", S_IRWXU | S_IRWXG | S_IRWXO);
        m_Log_File = std::fstream(log_file_name, std::ios::out);
        if(chmod(log_file_name.c_str(), S_IRWXU| S_IRWXG | S_IRWXO) != 0) {
            std::cout<<"[FATAL] Cannot chmod() on log files!"<<std::endl;
            perror("chmod");
        }
    }
#ifdef DEBUGMODE
    this->write_log(LOG_LEVEL::DEBUG, std::to_string(getuid()));
    this->write_log(LOG_LEVEL::DEBUG, std::to_string(getgid()));
#endif
}

void Logger::Log::write_log(LOG_LEVEL log_level, const std::string &message)
{
    std::string final_message;
    const std::vector log_level_indicators = {"[DEBUG]: ", "[WARNING]: ", "[ERROR]: ", "[FATAL]: "};
    const std::vector months = {
        "JAN",
        "FEB",
        "MAR",
        "APR",
        "MAY",
        "JUN",
        "JUL",
        "AUG",
        "SEP",
        "OCT",
        "NOV",
        "DEC"
    };
    time_t now = time(0);
    tm *ltm = localtime(&now);
    std::string time_string;
    time_string += std::to_string((ltm->tm_mday)) + "th ";
    time_string += months[ltm->tm_mon];
    time_string += " ";
    time_string += std::to_string(1900 + ltm->tm_year) + ", at ";
    time_string += std::to_string(ltm->tm_hour % 12 == 0 ? 12 : ltm->tm_hour % 12) + ":";
    time_string += std::to_string(ltm->tm_min) + ":";
    time_string += std::to_string(ltm->tm_sec) + " ";
    time_string += ltm->tm_hour >= 12 ? "PM" : "AM";

    final_message += log_level_indicators.at(log_level);
    final_message += message + " - ";
    final_message += time_string;

    this->m_Log_File<<final_message<<std::endl;
    if(log_level == LOG_LEVEL::FATAL)
    {
        exit(1);
    }
}

Logger::Log::~Log()
{
    this->m_Log_File.close();
}