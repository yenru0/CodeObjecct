def p_sum(cumulative, left, right):
    if 0 <= left <= right <= N:
        return cumulative[right] - cumulative[left]
    else:
        return 100000000000000000000000000


if __name__ == "__main__":
    N, S = map(int, input().split())
    A: list[int] = list(map(int, input().split()))
    CA: list[int] = [0]
    

    print(right - left)
