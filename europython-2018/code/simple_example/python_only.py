from math import sqrt
from random import random

def in_circle(x, y):
    return sqrt(x*x + y*y) <= 1.0

def calc(iterations):
    hits = 0
    for i in range(1, iterations):
        if in_circle(random(), random()):
            hits += 1

    pi = 4 * (hits / float(iterations))
    return pi
print(calc(10**9))
