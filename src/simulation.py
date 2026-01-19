# simulation.py
import random
from collections import deque

DIRECTIONS = ["N", "S", "W", "E"]

class Car:
    def __init__(self, direction):
        self.direction = direction
        self.wait_time = 0

class IntersectionSimulation:
    def __init__(self, spawn_prob=0.1, max_pass=1, seed=42):
        random.seed(seed)
        self.spawn_prob = spawn_prob
        self.max_pass = max_pass
        self.queues = {d: deque() for d in DIRECTIONS}
        self.green_direction = "N"

    def step(self):
        # 1. spawn cars
        for d in DIRECTIONS:
            if random.random() < self.spawn_prob:
                self.queues[d].append(Car(d))

        # 2. pass cars on green
        q = self.queues[self.green_direction]
        for _ in range(min(self.max_pass, len(q))):
            q.popleft()

        # 3. increment wait time
        for q in self.queues.values():
            for car in q:
                car.wait_time += 1

    def get_state(self):
        return {
            "queues": {d: len(q) for d, q in self.queues.items()},
            "green": self.green_direction,
        }

    def set_green(self, direction):
        self.green_direction = direction
