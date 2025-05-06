if __name__ == "__main__":
    s = input()
    cnt = 0
    temp = 0

    for i in range(1, len(s) - 1):
        if s[i : i + 2] == "))":
            cnt += temp
        elif s[i - 1 : i + 1] == "((":
            temp += 1
    print(cnt)
