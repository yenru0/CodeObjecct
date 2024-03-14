def continuous(a, b):
    return a + 1 == b


def solve(N: int, S: list[int]):
    x = []
    spares = []
    nums = list(S)
    nums.sort()

    x.append(nums[0])
    for n in nums[1:]:
        if continuous(x[-1], n):
            spares.append(n)
        else:
            x.append(n)
            while spares:
                if not continuous(x[-1], spares[0]):
                    x.append(spares.pop(0))
                else:
                    break

    if spares:
        k = -1
        l = len(x)
        while spares:
            if -l == k:
                x.insert(0, spares.pop(0))
                continue
            if continuous(spares[0], x[k]) or continuous(x[k - 1], spares[0]):
                k -= 1
            else:
                x.insert(k, spares.pop(0))

    return x


if __name__ == '__main__':
    N = int(input())
    S = list(map(int, input().split()))
    print(" ".join(map(str, solve(N, S))))
