N = int(input())
count = 0
now = 666


def isEnd(n):
    k = 100
    while k != 666:
        if n < 100:
            break
        k = n % 1000
        n //= 10
    else:
        return True


while count != N:
    if isEnd(now):
        count += 1
    now += 1
print(now - 1)  # Simple is the best
