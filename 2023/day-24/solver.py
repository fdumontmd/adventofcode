import sys
import z3

x0 = z3.Int('x')
y0 = z3.Int('y')
z0 = z3.Int('z')
a0 = z3.Int('a')
b0 = z3.Int('b')
c0 = z3.Int('c')
sol = z3.Int('sol')

s = z3.Solver()
s.add(sol == x0 + y0 + z0)

h = []
c = 0

for line in sys.stdin:
    line = line.strip()
    [pos, vec] = line.split("@")
    [x1, y1, z1] = pos.split(",")
    [a1, b1, c1] = vec.split(",")
    x1 = int(x1)
    y1 = int(y1)
    z1 = int(z1)
    a1 = int(a1)
    b1 = int(b1)
    c1 = int(c1)
    t = z3.Int("t" + str(c))
    h.append([x1, y1, z1, a1, b1, c1])
    s.add((x0 - x1) / (a1 - a0) > 0)
    s.add(t * (a1 - a0) == (x0 - x1))
    s.add(t * (b1 - b0) == (y0 - y1))
    s.add(t * (c1 - c0)== (z0 - z1))
    c += 1

s.check()
m  = s.model()

print(m[sol])
