from utils.util import run_client
from py_thoth.proto import mathop_pb2
from py_thoth.settings.connections import BaseThothObject


class ThothVector(BaseThothObject):
    def list_average(self, **kwargs):
        @run_client(self.remote_address)
        def __list_average(**kwargs):
            stub = kwargs["stub"]
            req = mathop_pb2.ListAverageOperationRequest(
                # x=a,
                operation_id=self.operation_id,
            )

            res = stub.ListAverage(req)
            return res.result_average

        return __list_average(**kwargs)

    def sort_list(self, ascending: bool = False, **kwargs):
        @run_client(self.remote_address)
        def __sort_list(ascending: bool = False, **kwargs):
            stub = kwargs["stub"]

            req = mathop_pb2.OrderListRequest(
                # x=a,
                ascending=ascending,
                operation_id=self.operation_id,
            )
            res = stub.OrderList(req)
            return res.result

        return __sort_list(ascending=ascending, **kwargs)

    def max_list(self, **kwargs):
        @run_client(self.remote_address)
        def __max_list(**kwargs):
            stub = kwargs["stub"]
            req = mathop_pb2.ListMaxRequest(
                # x=a,
                operation_id=self.operation_id,
            )

            res = stub.ListMax(req)
            return res.result

        return __max_list(**kwargs)

    def min_list(self, **kwargs):
        @run_client(self.remote_address)
        def __min_list(**kwargs):
            stub = kwargs["stub"]
            req = mathop_pb2.ListMinRequest(
                # x=a,
                operation_id=self.operation_id,
            )

            res = stub.ListMin(req)
            return res.result

        return __min_list(**kwargs)
