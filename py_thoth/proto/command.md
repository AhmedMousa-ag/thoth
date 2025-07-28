# How to generate proto files

From the py_thoth folder execute the following command:

```sh
python -m grpc_tools.protoc -I../grpc_proto --python_out=./proto --grpc_python_out=./proto ../grpc_proto/mathop.proto
```