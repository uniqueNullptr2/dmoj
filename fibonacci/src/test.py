def mod_sub(n, s, m):
    t = n - s
    t = t % m
    if t < 0:
        t = m - t
    return t


def fast_fib(n):
    if n == 0:
        return (0,1)
    elif n == 1:
        return (1,1)
    elif n < 200:
        n1 = 0
        n2 = 1
        for i in range(n):
            t = n1+n2
            n1 = n2
            n2 = t
        return (n1,n2)
    else:
        if n % 2 == 0:
            k, k1 = fast_fib(n//2)
            k2 = k * (2*k1 - k)
            k21 = k**2+k1**2
            return(k2,k21)
        else:
            k, k1 = fast_fib(n-1)
            k3 = k + k1
            return (k1,k3)

def fast_fib2(n):
    if n == 0:
        return (0,1)
    elif n == 1:
        return (1,1)
    elif n < 200:
        n1 = 0
        n2 = 1
        for i in range(n):
            t = ((n1%1000000007)+(n2 % 1000000007))% 1000000007
            n1 = n2
            n2 = t
        return (n1,n2)
    else:
        if n % 2 == 0:
            k, k1 = fast_fib2(n//2)
            if k > 2*k1:
                print(f"whaaat - {n} - {2*k1 - k} - {(2*k1 - k) % 1000000007}")
            k2 = (k * (2*k1 - k)) % 1000000007
            k21 = (k**2+k1**2) % 1000000007
            return(k2,k21)
        else:
            k, k1 = fast_fib2(n-1)
            k3 = (k+ k1) % 1000000007
            return (k1,k3)

n = 1000
# f, _ = fast_fib(n);
print(fast_fib(10000)[0] % 1000000007)
# print((f, f % 1000000007, fast_fib2(n)[1]))
# for i in range(10000):
#     m1 = fast_fib(i)[0] % 1000000007
#     m2 = fast_fib2(i)[0]
#     if m2 < 0:
#         print("what")
#     if m1 != m2:
#         print(f"{i} - {m1} - {m2}")
#         break