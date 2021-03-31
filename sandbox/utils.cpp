#include "utils.hpp"

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