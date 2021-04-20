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

#pragma once
#include <string>

struct config {
    std::string input_file;
    std::string output_file; // maybe misleading, but the sandbox program will store the output in the file specified in this string
    std::string source_file;
    std::string compile_command;
    std::string runtime_argv;
    std::string compiler_output_file;
    char*       binary;
    int         memory_limit;
    int         time_limit;
    int         child_uid;
    int         child_gid;
};
