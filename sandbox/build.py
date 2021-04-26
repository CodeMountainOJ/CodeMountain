#! /bin/env python
"""
 build.py is a Python script that simplifies CMake build process.
 Copyright (C) 2021 MD Gaziur Rahman Noor and contributors
 
 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 
 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with this program.  If not, see <https://www.gnu.org/licenses/>.
"""

import subprocess
import shlex
import os
from shutil import rmtree
from sys import argv

cmake_BUILD_COMMAND = 'cmake ../'
cmake_BUILD_COMMAND_DEBUG = 'cmake ../ -DDEBUG=true'
MAKE_BUILD_COMMAND  = 'make'
RUN_COMMAND         = 'YOUR_EXECUTABLE_HERE'

'''
    Ok, I(mdgaziur001) stole that from blender :^)
    I admit that :)
'''
class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'
'''
end of stolen code
'''

def hasValue(l: list, s: str) -> bool:
    for i in l:
        if i == s:
            return True
    return False

def CMake():
    print(bcolors.WARNING+"[STATUS] Spawning CMake process..."+bcolors.ENDC)

    stdin, stdout, stderr = '', '', ''
    if hasValue(argv, "--debug"):
        CMakeProcess = subprocess.Popen(shlex.split(cmake_BUILD_COMMAND_DEBUG), stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    else:
        CMakeProcess = subprocess.Popen(shlex.split(cmake_BUILD_COMMAND), stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    CMakeProcess.wait()
    if(CMakeProcess.returncode != 0):
        print(bcolors.FAIL+'[FAIL] CMake build process failed!'+bcolors.ENDC)
        print(CMakeProcess.stderr.read().decode())
        exit(1)
    else:
        print(bcolors.OKGREEN+"[SUCCESS] CMake build process finished!"+bcolors.ENDC)

def make():
    print(bcolors.WARNING+"[STATUS] Spawning Make process..."+bcolors.ENDC)

    stdin, stdout, stderr = '', '', ''
    MakeProcess = subprocess.Popen(shlex.split(MAKE_BUILD_COMMAND), stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    MakeProcess.wait()
    if(MakeProcess.returncode != 0):
        print(bcolors.FAIL+'[FAIL] Make build process failed!'+bcolors.ENDC)
        print(MakeProcess.stderr.read().decode())
        exit(1)
    else:
        print(bcolors.OKGREEN+"[SUCCESS] Make build process finished!"+bcolors.ENDC)

def mkdir():
    if(not os.path.exists('build')):
        os.mkdir('build')
    else:
        os.system('sudo rm -rf build')
        os.mkdir('build')

def cdtobuild():
    os.chdir('build')

def build():
    print(bcolors.WARNING+"[STATUS] Started building the executable..."+bcolors.ENDC)
    mkdir()
    cdtobuild()
    CMake()
    make()

build()