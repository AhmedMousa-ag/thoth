import functools
import dis

def math_tracer(func):
    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        print(args)
        print(kwargs)
        dis.dis(lambda: func(*args,**kwargs))
    return wrapper

# Example usage
@math_tracer
def calculate(x, y):
    return (x + y) * (x - y) / 2


print(calculate(x=5,y=600))


