# metrics.py
class Metrics:
    def __init__(self):
        self.total_wait = 0
        self.cars_count = 0
        self.history = []

    def update(self, sim):
        step_wait = 0
        step_cars = 0
        for q in sim.queues.values():
            for car in q:
                step_wait += car.wait_time
                step_cars += 1

        if step_cars > 0:
            self.history.append(step_wait / step_cars)

    def reset(self):
        self.__init__()
