N = int(input()); R = []
for i in sorted([input().split()for i in range(N)]):
    R.append(' '.join(i))
print('\n'.join(R), end='')