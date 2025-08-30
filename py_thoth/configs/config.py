import os
from .env_vars import REMOTE_ADDRESS


class Config:
    def __init__(self):
        self.remote_address = self.get_remote_address()

    def get_remote_address(self):
        remote_address = os.getenv(REMOTE_ADDRESS)
        if not remote_address:
            return ["localhost:50051"]
        return remote_address.split(",")
