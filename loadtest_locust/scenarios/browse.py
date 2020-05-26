from locust import SequentialTaskSet, task


class BrowseFootballers(SequentialTaskSet):

    @task
    def search(self):
        footballer_ids = set()
        response = self.client.get("/footballers").json()

        for footballer in response:
            footballer_ids.add(footballer["id"])

        for id in footballer_ids:
            with self.client.get("/footballers/%s" % id, name="/footballers/{id}", catch_response=True) as response:
                if response.status_code == 404:
                    response.success()

    @task
    def stop(self):
        self.interrupt()
