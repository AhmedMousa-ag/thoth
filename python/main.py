import grpc
from proto import mathop_pb2
from proto import mathop_pb2_grpc
import uuid


def run_client():
    """
    Connects to the gRPC server and calls the Calculate RPC.
    """
    with grpc.insecure_channel("localhost:50051") as channel:
        # Create a client stub. The class 'MathStub' is generated from the .proto file.
        stub = mathop_pb2_grpc.MathOpsStub(channel)
        # Define your matrices as Python lists of lists
        matrix_one_data = [[1.0, 2.0, 3.0], [1.0, 2.0, 3.0]]
        matrix_two_data = [[1.0, 2.0], [1.0, 2.0], [3.0, 4.0]]

        # Convert Python lists to proto Matrix objects
        matrix_a_proto = mathop_pb2.Matrix()
        for row_data in matrix_one_data:
            matrix_a_proto.rows.add(values=row_data)

        matrix_b_proto = mathop_pb2.Matrix()
        for row_data in matrix_two_data:
            matrix_b_proto.rows.add(values=row_data)

        # Create the request using the proto Matrix objects
        req = mathop_pb2.MatrixOperationRequest(
            matrix_a=matrix_a_proto,
            matrix_b=matrix_b_proto,
            operation_id=str(uuid.uuid4()),
        )

        res = stub.MatrixMultiply(req)
        print(res)


if __name__ == "__main__":
    run_client()
