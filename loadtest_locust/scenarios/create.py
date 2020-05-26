from json import dumps
from locust import TaskSet, task
from random import randrange


class CreateFootballers(TaskSet):

    @task(10)
    def create(self):
        payload = {
            "firstName": "Test",
            "lastName": "Player %d" % randrange(10000),
            "position": "Test %d" % randrange(10)
        }

        self.client.post("/footballers", 
            headers = {"Content-Type": "application/json"},
            data = dumps(payload))

    @task
    def stop(self):
        self.interrupt()
