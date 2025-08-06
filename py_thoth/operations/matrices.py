from proto import mathop_pb2
import uuid
from typing import List
from utils.util import run_client


@run_client
def matrix_multiply(a: List[List[float]], b: List[List[float]], **kwargs):
    stub = kwargs["stub"]
    operation_id = kwargs.get("operation_id", str(uuid.uuid4()))
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
        operation_id=operation_id,
    )

    res = stub.MatrixMultiply(req)
    return res
