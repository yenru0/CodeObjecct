def solve(N, A):
    stack_i = []
    stack_v = []
    t = [-1] * N

    for i in range(N):
        if not stack_v:
            stack_v.append(A[i])
            stack_i.append(i)
        else:
            if A[i] > stack_v[-1]:
                for j in range(len(stack_v) - 1, -1, -1):
                    if stack_v[j] < A[i]:
                        t[stack_i[j]] = A[i]
                        stack_v.pop()
                        stack_i.pop()
                    else:
                        break
            stack_v.append(A[i])
            stack_i.append(i)

    return " ".join(map(str, t))


if __name__ == '__main__':
    print(solve(int(input()), list(map(int, input().split()))))
