def X(N, K, A) -> int:
    A.sort()
    start = 0
    end = max(A)
    mid = start
    while start < end:
        mid = (start + end) // 2
        w = sum(x - mid for x in A if x > mid)
        if w > K:
            start = mid + 1
        else:
            end = mid
    return start


if __name__ == "__main__":
    N, K = map(int, input().split())
    A = list(map(int, input().split()))
    print(X(N, K, A))
