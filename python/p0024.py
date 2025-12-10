import numpy as np
import matplotlib.pyplot as plt
from itertools import permutations

nums = [i for i in range(10)]
vals = []
for i in range(len(nums)):
    vals.append(str(nums[i]))

perms = list(permutations(vals))
sep = ''
ints = []
for i in range(len(perms)):
    ints.append(int(sep.join(perms[i])))

print(ints[1000000-1])