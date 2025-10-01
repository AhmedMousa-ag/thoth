import grpc
import functools
from configs.config import Config
import random
from py_thoth.proto import mathop_pb2_grpc


def run_client(func):
    """
    Decorator that connects to the gRPC server and injects the stub into the decorated function.
    """

    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        conn_options = [
            ("grpc.max_send_message_length", 1 * 1000 * 1024 * 1024),  # 5GB
            ("grpc.max_receive_message_length", 1 * 1000 * 1024 * 1024),  # 5GB
        ]
        with grpc.insecure_channel(
            random.choice(Config().remote_address), options=conn_options
        ) as channel:
            stub = mathop_pb2_grpc.MathOpsStub(channel)
            # Inject the stub into kwargs
            kwargs["stub"] = stub
            return func(*args, **kwargs)

    return wrapper
