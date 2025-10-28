import random
import time
from py_thoth.operations.vector import ThothVector
from py_thoth.settings.connections import change_remote_address

change_remote_address(["localhost:50051"])  # single node for testing

# Large list average
list_to_compute = [float(random.randint(1, 100)) for _ in range(5_000_000)]
vector_obj = ThothVector(list_to_compute)

start_time = time.time()
average_result = vector_obj.list_average()
end_time = time.time()

print(f"Computed average: {average_result:.2f}")
print(f"Time taken: {end_time - start_time:.2f} seconds")

# Validate against Python's built-in calculation
expected_average = round(sum(list_to_compute) / len(list_to_compute), 2)
assert round(average_result, 2) == expected_average
