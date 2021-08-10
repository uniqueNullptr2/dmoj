from subprocess import Popen, PIPE, STDOUT
import sys
from random import randint
x = Popen(['/home/phil/Documents/dmoj/snowflakes/target/debug/snowflakes'], stdout=sys.stdout, stdin=PIPE, stderr=STDOUT)

l = 100000

x.stdin.write(f"{l}\n".encode())
x.stdin.flush()

for i in range(l):
    s = ""
    for e in range(8):
        s += f"  {randint(100000000, 100000000)} "
    s += "\n"
    x.stdin.write(s.encode())
    x.stdin.flush()



