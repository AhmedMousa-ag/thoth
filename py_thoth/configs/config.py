class Config:
    remote_address = "localhost:50051"

    def change_remote_address(self, new_address: str):
        self.remote_address = new_address


CONFIG = Config()
