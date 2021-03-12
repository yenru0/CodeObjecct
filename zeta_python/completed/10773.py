def solve(K, I):
    stack = list()
    for i in I:
        if i:
            stack.append(i)
        else:
            stack.pop()
    return sum(stack)


if __name__ == '__main__':
    K = int(input())
    print(solve(K, [int(input()) for _ in range(K)]))
