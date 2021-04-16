#include "signal.hpp"
#include <csignal>
#include <unistd.h>

void systemError()
{
    raise(SIGUSR1);
    _exit(1);
}