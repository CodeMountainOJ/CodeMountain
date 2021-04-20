/* 
 *  This is the default license template.
 *  
 *  File: execve.c
 *  Author: mdgaziur001
 *  Copyright (c) 2021 mdgaziur001
 *  
 *  To edit this license information: Press Ctrl+Shift+P and press 'Create new License Template...'.
 */

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