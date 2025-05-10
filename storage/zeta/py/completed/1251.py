S = input()
MAX = '~'*len(S)
for i in range(1,len(S)-1):
    for j in range(i+1, len(S)):
        first, second, last = S[:i][::-1], S[i:j][::-1], S[j:][::-1]
        if first + second + last < MAX:
            MAX = first + second + last
print(MAX)