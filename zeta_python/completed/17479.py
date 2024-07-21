import sys

input = sys.stdin.readline
if __name__ == "__main__":
    A, B, C = map(int, input().split())
    normal = dict()
    special = dict()
    service = []

    for _ in range(A):
        name, price = input().rstrip().split()
        price = int(price)
        normal[name] = price
    for _ in range(B):
        name, price = input().rstrip().split()
        price = int(price)
        special[name] = price
    for _ in range(C):
        name = input().strip()
        service.append(name)

    N = int(input())
    normal_price = 0
    total_price = 0
    service_count = 0
    for _ in range(N):
        order = input().strip()
        if order in normal:
            p = normal[order]
            normal_price += p
            total_price += p
        elif order in special:
            p = special[order]
            total_price += p
        else:
            service_count += 1
    flag = True
    if normal_price < 20000 and total_price - normal_price > 0:
        flag = False
    else:
        if (total_price < 50000 and service_count > 0) or service_count > 1:
            flag = False

    print("Okay") if flag else print("No")
