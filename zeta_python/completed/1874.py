def solve(N, I):
    ret = []
    stack = []
    c = 0
    for i, x in enumerate(I):
        while c < x:
            c += 1
            stack.append(c)
            ret.append("+")

        if stack[-1] == x:
            stack.pop()
            ret.append("-")

        else:
            return "NO"
    return "\n".join(ret)


if __name__ == '__main__':
    N = int(input())
    print(solve(N, list(int(input()) for _ in range(N))))
