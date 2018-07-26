from math import sqrt
from random import random

def in_circle(double x, double y):
    return sqrt(x*x + y*y) <= 1.0

def calc(long iterations):
    cdef long hits
    cdef double pi

    hits = 0
    for _ in range(1, iterations):
        if in_circle(random(), random()):
            hits += 1

    pi = 4 * (hits / float(iterations))
    return pi
print(calc(10**9))
