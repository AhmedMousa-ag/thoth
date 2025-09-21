import uuid
from typing import List
from utils.util import run_client
from py_thoth.proto import mathop_pb2

@run_client
def list_average(a: List[float], **kwargs):
    stub = kwargs["stub"]
    operation_id = kwargs.get("operation_id", str(uuid.uuid4()))
    req = mathop_pb2.ListAverageOperationRequest(
        x=a,
        operation_id=operation_id,
    )

    res = stub.ListAverage(req)
    return res.result_average

@run_client
def sort_list(a: List[float], ascending : bool = False, **kwargs):
    stub = kwargs["stub"]
    operation_id = kwargs.get("operation_id", str(uuid.uuid4()))
    req = mathop_pb2.OrderListRequest(
        x=a,
        ascending=ascending,
        operation_id=operation_id,
        )
    res = stub.OrderList(req)
    return res.result