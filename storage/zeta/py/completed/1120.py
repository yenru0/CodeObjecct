A, B = input().split()


def get(a, b):
    cnt = 0
    for f, s in zip(a, b):
        if f != s:
            cnt += 1
    return cnt


print(min(get(A, B[i:i+len(A)+1]) for i in range(len(B) - len(A)+1)))
