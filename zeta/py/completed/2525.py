h, m = map(int, input().split())
d = int(input())

m_o = (m + d) // 60
m_new = (m + d) % 60

h_new = (h + m_o) % 24
print(h_new, m_new)