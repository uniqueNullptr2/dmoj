from random import randint, choice

N = 10000000
Q = 10000

QQ = ["T", "L", "F", "S"]
QQQ = [">", "<", "."]

s = set()
l = []

def ran_set():
    while True:
        t = choice(l)
        if t in s:
            return t

def gen_event():
    t = randint(-1*N, N)
    s.add(t)
    l.append(t)
    return f"{t} {randint(-1*N, N)}"

def gen_query():
    q1 = choice(QQ)
    q1 = "S"
    if q1 == "F":
        return f"{q1} {choice(QQQ)} {randint(-1*N, N)}"
    elif q1 == "S":
        return f"{q1} {randint(-1*N, N)}"
    elif q1 == "T":
        t = ran_set()
        s.remove(t)
        tt = randint(-1*N, N)
        s.add(tt)
        l.append(tt)
        return f"{q1}  {t} {tt}"
    else:
        return f"{q1}  {ran_set()} {randint(-1*N, N)}"

n = randint(0, N)
q = randint(0, Q)

lines = []
lines.append(str(n) + "\n")
for i in range(n):
    lines.append(gen_event() + "\n")
lines.append(str(q) + "\n")
for i in range(q):
    lines.append(gen_query() + "\n")

with open("test_input2", "w") as f:
    f.writelines(lines)