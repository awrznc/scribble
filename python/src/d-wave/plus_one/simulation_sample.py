import wildqat as wq

a = wq.opt()
a.qubo = [[-1,2],[0,-1]]

print(a.sa())
