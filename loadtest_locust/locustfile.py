from locust import HttpUser, task, constant
from scenarios.browse import BrowseFootballers
from scenarios.create import CreateFootballers


class Creator(HttpUser):
    tasks = [CreateFootballers]
    wait_time = constant(5)
    weight = 1

class Browser(HttpUser):
    tasks = [BrowseFootballers]
    wait_time = constant(0)
    weight = 1
