if __name__ == "__main__":
    N, M = map(int, input().split())
    A = list(map(int, input().split()))
    A.sort()
    start = 0
    end = N - 1
    cnt = 0
    while start < end:
        if A[start] + A[end] >= M:
            cnt += 1
            end -= 1
            start += 1
        else:
            start += 1
    print(cnt)
