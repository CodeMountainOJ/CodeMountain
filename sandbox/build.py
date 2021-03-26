#! /bin/python
import subprocess
import shlex
import os
from shutil import rmtree

cmake_BUILD_COMMAND = 'cmake ../'
MAKE_BUILD_COMMAND  = 'make'
RUN_COMMAND         = './codemountain_sandbox'

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

def CMake():
    print(bcolors.WARNING+"[STATUS] Spawning CMake process..."+bcolors.ENDC)

    stdin, stdout, stderr = '', '', ''
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
        rmtree('build')
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