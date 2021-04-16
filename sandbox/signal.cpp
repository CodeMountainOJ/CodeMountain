#include "signal.hpp"
#include <csignal>

void systemError()
{
    raise(SIGUSR1);
}