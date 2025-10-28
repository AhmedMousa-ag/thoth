from setuptools import setup, find_packages
import subprocess
import os


def get_git_version_from_tags(default_version="0.0.1", remove_v_prefix=True):
    """Try to get the exact tag if on one (e.g., 'v1.2.3')"""

    output = subprocess.run(
        ["git", "describe", "--tags", "--abbrev=0"],
        stderr=subprocess.DEVNULL,
        text=True,
        cwd=os.path.dirname(os.path.abspath(__file__)),
    )
    version = output.stdout.strip() if output.stdout else default_version

    if remove_v_prefix and version.startswith("v"):
        version = version[1:]

    return version


def _packages():
    # Discover subpackages in the current directory (configs, operations, ...)
    discovered = [p for p in find_packages(where=".") if p]
    # Prefix them so they're installed under the 'py_thoth' namespace
    prefixed = [f"py_thoth.{p}" for p in discovered]
    # Always include the root package itself
    return ["py_thoth", *prefixed]


setup(
    name="py_thoth",
    version=get_git_version_from_tags(),
    # Map the current directory to the 'py_thoth' package name
    package_dir={"py_thoth": "."},
    packages=_packages(),
    include_package_data=True,
    install_requires=[
        "grpcio==1.74.0",
        "grpcio-tools==1.74.0",
    ],
    python_requires=">=3.8",
)
