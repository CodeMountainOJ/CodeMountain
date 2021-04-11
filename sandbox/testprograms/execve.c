/*

Test for execve blocking(seccomp)

Should not output anything if seccomp actually works.

*/
#include <stdio.h>
#include <unistd.h>

int main()
{
    printf("You shouldn't see the output of 'uname -r'\n");
    const char* argv[] = { "uname", "-r", NULL };
    execve(argv[0], &argv[0], __environ);
}