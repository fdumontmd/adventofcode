from itertools import combinations
import numpy as np
from math import prod

exterior3 = lambda u,v,w: u[0]*v[1]*w[2] + u[1]*v[2]*w[0] + u[2]*v[0]*w[1] - u[0]*v[2]*w[1] - u[1]*v[0]*w[2] - u[2]*v[1]*w[0]
exterior2 = lambda v,w: [v[0]*w[1] - v[1]*w[0], v[1]*w[2] - v[2]*w[1], v[2]*w[0] - v[0]*w[2]]
sub = lambda u,v: [a - b for a, b in zip(u, v)]
p,v = 0,1

d = [[[int(i) for i in s.split(',')]
                for s in l.split('@')]
                for l in open(0)]

equation = lambda i,j: [*exterior2(sub(d[i][v],d[j][v]), sub(d[i][p],d[j][p])), -exterior3(d[i][p],d[i][v],d[j][p]) - exterior3(d[j][p],d[j][v],d[i][p])]

# a bit of an abuse here: exterior3(p, vij, pij) != p * exterior2(vij, pij) ; the components will be shifted. The sum is the same though
argumented_matrix = np.array([equation(i,j) for i,j in combinations([0,1,2],2)]).astype('float64')

p = np.linalg.solve(argumented_matrix[:,:-1], argumented_matrix[:,-1])
print(p)

print(sum(p))

