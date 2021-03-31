#pragma once
#include <string>

struct result
{
    bool compileErrors = false;
    bool runtimeErrors = false;
    bool timeLimitExceeded = false;
    bool memoryLimitExceeded = false;
    bool systemError = false;
    int  spentTime;
    int  usedMemory;
};