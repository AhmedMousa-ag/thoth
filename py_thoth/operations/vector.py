from proto import mathop_pb2
import uuid
from typing import List
from utils.util import run_client


@run_client
def list_average(a: List[float], stub):
    req = mathop_pb2.ListAverageOperationRequest(
        result_average=a,
        operation_id=str(uuid.uuid4()),
    )

    res = stub.ListAverage(req)
    return res
