"""Define important attributes for sqlite connection."""

import os
from pathlib import Path

DEFAULT_SQLITE_LOCATION = Path(
    os.environ.get("VRS_VCF_INDEX", Path.home() / ".local" / "share" / "vrsix.db")
)
