# visualization.py
import pygame

# цвета
WHITE = (255, 255, 255)
BLACK = (0, 0, 0)
RED = (200, 50, 50)
GREEN = (50, 200, 50)
CAR_COLOR = (50, 150, 250)
ROAD_COLOR = (50, 50, 50)

# координаты перекрестка
CENTER_X, CENTER_Y = 200, 150
ROAD_WIDTH = 40
CAR_SIZE = 20
GAP = 5  # расстояние между машинами

# направления и смещения для машин
DIRECTION_INFO = {
    "N": {"dx": 0, "dy": 1, "start": (CENTER_X - ROAD_WIDTH//2, 0)},
    "S": {"dx": 0, "dy": -1, "start": (CENTER_X + ROAD_WIDTH//2 - CAR_SIZE, 300 - CAR_SIZE)},
    "W": {"dx": 1, "dy": 0, "start": (0, CENTER_Y + ROAD_WIDTH//2 - CAR_SIZE)},
    "E": {"dx": -1, "dy": 0, "start": (400 - CAR_SIZE, CENTER_Y - ROAD_WIDTH//2)},
}

def draw_roads(screen):
    # вертикальная
    pygame.draw.rect(screen, ROAD_COLOR, (CENTER_X - ROAD_WIDTH//2, 0, ROAD_WIDTH, 300))
    # горизонтальная
    pygame.draw.rect(screen, ROAD_COLOR, (0, CENTER_Y - ROAD_WIDTH//2, 400, ROAD_WIDTH))

def draw_cars(screen, sim):
    for d, q in sim.queues.items():
        info = DIRECTION_INFO[d]
        x, y = info["start"]
        dx, dy = info["dx"], info["dy"]
        for idx, car in enumerate(q):
            cx = x + dx * idx * (CAR_SIZE + GAP)
            cy = y + dy * idx * (CAR_SIZE + GAP)
            pygame.draw.rect(screen, CAR_COLOR, (cx, cy, CAR_SIZE, CAR_SIZE))

def draw_traffic_light(screen, sim):
    for d in DIRECTION_INFO:
        info = DIRECTION_INFO[d]
        x, y = info["start"]
        color = GREEN if sim.green_direction == d else RED
        pygame.draw.circle(screen, color, (x + CAR_SIZE//2, y + CAR_SIZE//2), 10)

def draw(screen, sim, mode):
    screen.fill(BLACK)
    draw_roads(screen)
    draw_cars(screen, sim)
    draw_traffic_light(screen, sim)

    font = pygame.font.SysFont(None, 24)
    text = font.render(mode, True, WHITE)
    screen.blit(text, (10, 10))
