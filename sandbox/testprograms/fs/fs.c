/* 
 *  This is the default license template.
 *  
 *  File: fs.c
 *  Author: mdgaziur001
 *  Copyright (c) 2021 mdgaziur001
 *  
 *  To edit this license information: Press Ctrl+Shift+P and press 'Create new License Template...'.
 */

#include <stdio.h>
#include <errno.h>

int main()
{
    if(remove("../build/program") == -EPERM) // this wouldn't work if it wasn't ../build/program as it will be ran from the client directory while cargo testing
    {
        perror("remove");
    }
}