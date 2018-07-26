from distutils.core import setup
from Cython.Build import cythonize

setup(
    extra_compile_args=["-O3"],
    ext_modules = cythonize("with_cython.pyx")
)
