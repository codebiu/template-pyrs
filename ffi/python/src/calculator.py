from ctypes import *
from typing import Optional
import os
import sys

# 获取动态库路径
def _get_lib_path() -> str:
    if sys.platform == "win32":
        return "tempalte_lib.dll"
    elif sys.platform == "darwin":
        return "libtempalte_lib.dylib"
    else:
        return "libtempalte_lib.so"

# 加载动态库
_lib = CDLL(_get_lib_path())

class Calculator:
    def __init__(self):
        self._ptr = _lib.calculator_new()
        if not self._ptr:
            raise MemoryError("Failed to create calculator")

    def __del__(self):
        if hasattr(self, '_ptr') and self._ptr:
            _lib.calculator_free(self._ptr)
            self._ptr = None

    def add(self, a: float, b: float) -> float:
        _lib.calculator_add.restype = c_double
        return _lib.calculator_add(self._ptr, c_double(a), c_double(b))

    def subtract(self, a: float, b: float) -> float:
        _lib.calculator_subtract.restype = c_double
        return _lib.calculator_subtract(self._ptr, c_double(a), c_double(b))

    def multiply(self, a: float, b: float) -> float:
        _lib.calculator_multiply.restype = c_double
        return _lib.calculator_multiply(self._ptr, c_double(a), c_double(b))

    def divide(self, a: float, b: float) -> float:
        _lib.calculator_divide.restype = c_double
        error = POINTER(c_char_p)()
        result = _lib.calculator_divide(self._ptr, c_double(a), c_double(b), byref(error))
        if error.contents:
            error_message = string_at(error.contents).decode('utf-8')
            raise ZeroDivisionError(error_message)
        return result