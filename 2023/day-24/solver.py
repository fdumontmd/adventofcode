import sys
import z3

# Int is shockingly slow; Real prints the solution
# instantly
x0 = z3.Real('x')
y0 = z3.Real('y')
z0 = z3.Real('z')
a0 = z3.Real('a')
b0 = z3.Real('b')
c0 = z3.Real('c')
sol = z3.Real('sol')

s = z3.Solver()
s.add(sol == x0 + y0 + z0)

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
    t = z3.Real("t" + str(c))
    s.add(t >= 0)
    s.add(x0 + t * a0 == x1 + t * a1)
    s.add(y0 + t * b0 == y1 + t * b1)
    s.add(z0 + t * c0 == z1 + t * c1)
    c += 1
    if c > 8:
        break

s.check()
m  = s.model()

print(m[x0])
print(m[y0])
print(m[z0])

print(m[sol])
