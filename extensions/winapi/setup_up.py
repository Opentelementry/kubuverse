from setuptools import setup, Extension
import sys
import platform

# Detect compiler/arch details
extra_compile_args = []
if platform.system() == "Windows":
    extra_compile_args = ["/std:c11", "/O2"]
else:
    extra_compile_args = ["-std=c11", "-O3"]

kubu_winapi_module = Extension(
    "kubu_winapi",
    sources=["kubu_winapi.c"],
    libraries=["kernel32"],  # for GetOverlappedResult and WinAPI calls
    extra_compile_args=extra_compile_args,
)

setup(
    name="kubu_winapi",
    version="0.1.0",
    description="Custom Kubuverse WinAPI extension for async I/O and system integration",
    ext_modules=[kubu_winapi_module],
)
