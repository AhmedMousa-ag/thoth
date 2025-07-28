import grpc
from proto import mathop_pb2
from proto import mathop_pb2_grpc
import uuid
from typing import List


def matrix_multiply(a: List[List[float]], b: List[List[float]], stub):
    # Convert Python lists to proto Matrix objects
    matrix_a_proto = mathop_pb2.Matrix()
    for row_data in a:
        matrix_a_proto.rows.add(values=row_data)

    matrix_b_proto = mathop_pb2.Matrix()
    for row_data in b:
        matrix_b_proto.rows.add(values=row_data)

    # Create the request using the proto Matrix objects
    req = mathop_pb2.MatrixOperationRequest(
        matrix_a=matrix_a_proto,
        matrix_b=matrix_b_proto,
        operation_id=str(uuid.uuid4()),
    )

    res = stub.MatrixMultiply(req)
    return res


def list_average(a: List[float], stub):
    req = mathop_pb2.ListAverageOperationRequest(
        result_average=a,
        operation_id=str(uuid.uuid4()),
    )

    res = stub.ListAverage(req)
    return res


def run_client():
    """
    Connects to the gRPC server and calls the Calculate RPC.
    """
    with grpc.insecure_channel("localhost:50051") as channel:
        # Create a client stub. The class 'MathStub' is generated from the .proto file.
        stub = mathop_pb2_grpc.MathOpsStub(channel)
        # Define your matrices as Python lists of lists
        a = [[1.0, 2.0, 3.0], [1.0, 2.0, 3.0]]
        b = [[1.0, 2.0], [1.0, 2.0], [3.0, 4.0]]
        print(matrix_multiply(a, b, stub))
        print(list_average([5, 6, 5, 5, 7, 9, 5, 7, 2, 5, 5, 5, 5.7, 9]))


if __name__ == "__main__":
    run_client()
