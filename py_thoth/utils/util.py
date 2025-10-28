import grpc
import functools
from configs.config import Config
import random
from py_thoth.proto import mathop_pb2_grpc


def run_client(remote_address: str | None = None):
    """
    Decorator that connects to the gRPC server and injects the stub into the decorated function.

    Args:
        remote_address: Optional remote address to use. If None, will use from Config or kwargs.
    """

    def decorator(func):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            conn_options = [
                ("grpc.max_send_message_length", 1 * 1000 * 1024 * 1024),  # 5GB
                ("grpc.max_receive_message_length", 1 * 1000 * 1024 * 1024),  # 5GB
            ]
            address = remote_address or random.choice(Config().remote_address)
            with grpc.insecure_channel(address, options=conn_options) as channel:
                stub = mathop_pb2_grpc.MathOpsStub(channel)
                # Inject the stub into kwargs
                kwargs["stub"] = stub
                return func(*args, **kwargs)

        return wrapper

    return decorator
