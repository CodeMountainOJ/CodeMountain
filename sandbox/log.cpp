#include "log.hpp"
#include <iostream>
#include <vector>
#include <ctime>

Logger::Log::Log(std::string log_file_name)
    : m_Log_File(std::fstream(log_file_name, std::ios::out))
{
    if(!this->m_Log_File)
    {
        std::cout<<"[FATAL] Failed to open or create log file!"<<std::endl;
        exit(2);
    }

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
        "JUN"
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
    time_string += std::to_string((1+(ltm->tm_mon))) + "th ";
    time_string += months[ltm->tm_mon];
    time_string += " ";
    time_string += std::to_string(1900 + ltm->tm_year) + ", at ";
    time_string += std::to_string(ltm->tm_hour % 12 == 0 ? 12 : ltm->tm_hour % 12) + ":";
    time_string += std::to_string(ltm->tm_min) + ":";
    time_string += std::to_string(ltm->tm_sec) + " ";
    time_string += ltm->tm_hour >= 12 ? "PM" : "AM";

    final_message += log_level_indicators.at(log_level - 1);
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