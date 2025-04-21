from locust import HttpUser, task, between, events

class Test(HttpUser):
    wait_time = between(0.1, 0.5)

    @task
    def test(self):
        self.client.get("/order/query-one?id=1")


