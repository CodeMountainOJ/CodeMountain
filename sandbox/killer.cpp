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