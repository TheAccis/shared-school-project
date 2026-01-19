# controller.py
from simulation import DIRECTIONS

class FixedController:
    def __init__(self, period=50):
        self.period = period
        self.timer = 0
        self.index = 0

    def decide(self, sim):
        self.timer += 1
        if self.timer >= self.period:
            self.timer = 0
            self.index = (self.index + 1) % len(DIRECTIONS)
        return DIRECTIONS[self.index]

class AdaptiveController:
    def __init__(self, threshold=2):
        self.threshold = threshold

    def decide(self, sim):
        queues = sim.get_state()["queues"]
        best = max(queues, key=lambda d: queues[d])
        return best
