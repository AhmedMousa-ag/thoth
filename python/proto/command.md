# How to generate proto files

From the main folder execute the following command:

```sh
python -m grpc_tools.protoc -Igrpc_proto --python_out=python/proto --grpc_python_out=python/proto grpc_proto/mathop.proto
```