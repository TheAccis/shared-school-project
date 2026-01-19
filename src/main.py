# main.py
import pygame
from simulation import IntersectionSimulation
from controller import FixedController, AdaptiveController
from metrics import Metrics
from visualization import draw

pygame.init()
screen = pygame.display.set_mode((400, 300))
clock = pygame.time.Clock()

sim = IntersectionSimulation()
fixed = FixedController()
adaptive = AdaptiveController()
metrics = Metrics()

mode = "fixed"
running = True

while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
        if event.type == pygame.KEYDOWN:
            if event.key == pygame.K_SPACE:
                mode = "adaptive" if mode == "fixed" else "fixed"
            if event.key == pygame.K_r:
                sim = IntersectionSimulation()
                metrics.reset()

    controller = fixed if mode == "fixed" else adaptive
    sim.set_green(controller.decide(sim))
    sim.step()
    metrics.update(sim)

    draw(screen, sim, mode)
    pygame.display.flip()
    clock.tick(5)

pygame.quit()