#pragma once
#include "run.hpp"
#include "result.hpp"

class killer
{
public:
    killer(config*, result*, pid_t);
    void cancel();
private:
    bool m_IsCleared = false;
};