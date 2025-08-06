class Config:
    remote_address = ["localhost:50051"]

    def setup_remote_address(self, new_address: list[str]):
        if isinstance(new_address, str):
            new_address = [new_address]
        self.remote_address = new_address


CONFIG = Config()
