import grpc
from proto import mathop_pb2_grpc
import functools
from configs.config import CONFIG


def run_client(func):
    """
    Decorator that connects to the gRPC server and injects the stub into the decorated function.
    """

    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        with grpc.insecure_channel(CONFIG.remote_address) as channel:
            stub = mathop_pb2_grpc.MathOpsStub(channel)
            # Inject the stub into kwargs
            kwargs["stub"] = stub
            return func(*args, **kwargs)

    return wrapper
