# API Document

This document provides an overview of the API available in the Thoth distributed computing framework.
Thoth uses gRPC for communication between nodes and the gRPC Client to send tasks to the cluster. Currently, the primary and only interface for interacting with Thoth is through the `py_thoth` Python package.

We can divide the API into several categories based on the type of operations they perform, so far we only have operations for lists and matrices.

## Python Package: py_thoth

The `py_thoth` package provides a set of operations that can be executed on a Thoth cluster. For details on how to install and set up the package, please refer to the [Getting Started](../README.md#usage) section in the main README file.


For detailed Python API documentation, please refer to the [py_thoth API Documentation](./api/py_thoth.md) document.