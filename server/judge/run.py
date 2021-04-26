#! /bin/env python
# 
#  CodeMountain is a free and open source online judge open for everyone
#  Copyright (C) 2021 MD Gaziur Rahman Noor and contributors
#  
#  This program is free software: you can redistribute it and/or modify
#  it under the terms of the GNU General Public License as published by
#  the Free Software Foundation, either version 3 of the License, or
#  (at your option) any later version.
#  
#  This program is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU General Public License for more details.
#  
#  You should have received a copy of the GNU General Public License
#  along with this program.  If not, see <https://www.gnu.org/licenses/>.
#

import os
import sys

if sys.argv.count("--no-build") == 0:
    os.chdir("../../sandbox")
    os.system("./build.py")
    os.chdir("../server/judge")


os.environ["JUDGESERVER_SANDBOX_EXECUTABLE"] = "../../sandbox/build/codemountain_sandbox"
os.system("cargo run")