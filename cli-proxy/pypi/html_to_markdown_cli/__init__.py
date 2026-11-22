"""CLI proxy package."""

from importlib import metadata

try:
    __version__ = metadata.version("html-to-markdown-cli")
except metadata.PackageNotFoundError:
    __version__ = "0+unknown"
