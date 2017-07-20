import random

clauses = 30
variables = 5

print("p cnf",variables, clauses)
for i in range(0,clauses):
    r1 = random.randint(1, variables) * (random.randint(0,1)*2-1)
    r2 = random.randint(1, variables) * (random.randint(0,1)*2-1)
    r3 = random.randint(1, variables) * (random.randint(0,1)*2-1)
    print(r1,r2,r3,0)

