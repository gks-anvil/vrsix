"""Tools for constructing and interacting with sqlite-based indexing of VRS-annotated VCFs."""

from importlib.metadata import PackageNotFoundError, version

try:
    __version__ = version("thera-py")
except PackageNotFoundError:
    __version__ = "unknown"
finally:
    del version, PackageNotFoundError
