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

#include <thread>
#include <chrono>
#include <signal.h>
#include "killer.hpp"

killer::killer(config *sandbox_config, result *result_struct, pid_t victim)
{
    m_IsCleared = false;
    std::thread t([&] {
        if(m_IsCleared) return;
        std::this_thread::sleep_for(std::chrono::seconds(sandbox_config->time_limit));
        if(m_IsCleared) return;
        kill(victim, SIGKILL);
        result_struct->timeLimitExceeded = true;
    });
    t.detach();
}

void killer::cancel()
{
    m_IsCleared = true;
}