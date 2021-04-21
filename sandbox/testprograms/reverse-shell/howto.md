# THIS GUIDE SHOWS HOW TO TEST REVERSE SHELL BLOCKING CAPABILITY OF THIS SANDBOX

- First install netcat if isn't
- Then run ```nc -lvnp 8888``` on your computer
- Then run ```python program.py```
- If a shell appears, that means the sandbox couldn't prevent it from making reverse shell
- If that happens, report it immediately by making a issue in the repo