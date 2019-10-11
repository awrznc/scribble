import wildqat as wq
import numpy as np

H_A = np.array([[-2,1,1,0,0],[0,-2,0,1,0],[0,0,-2,1,0],[0,0,0,-3,1],[0,0,0,0,-1]])
H_B = np.array([[1,0,0,0,0],[0,1,0,0,0],[0,0,1,0,0],[0,0,0,1,0],[0,0,0,0,1]])

a = wq.opt()
a.qubo = H_A + H_B * 0.5
print(a.sa())
