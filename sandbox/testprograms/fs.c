#include <stdio.h>
#include <errno.h>

int main()
{
    if(remove("program") == -EPERM)
    {
        perror("remove");
    }
}