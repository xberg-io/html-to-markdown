"""Overlay only the verified generated bool-to-int adaptation on release source."""

import argparse
import hashlib
import json
import re
import subprocess
from pathlib import Path

CONVERTER = "packages/csharp/HtmlToMarkdown/HtmlToMarkdownConverter.cs"
CALL = "NativeMethods.ConvertWithAccessibleNames(html, useAccessibleNames, optionsHandle)"
HASH = re.compile(r"(?m)^// alef:hash:[0-9a-f]{64}$")


def verify_adaptation(original: str, corrected: str) -> int:
    """Reject missing work, inverted policy and every unrelated source change."""
    if len(HASH.findall(original)) != 1 or len(HASH.findall(corrected)) != 1:
        raise ValueError("Managed recovery refused: expected one generated hash per file")
    count = original.count(CALL)
    expected = original.replace(CALL, CALL.replace("useAccessibleNames", "(useAccessibleNames ? 1 : 0)"))
    if count != 2 or HASH.sub("// alef:hash:verified", expected) != HASH.sub("// alef:hash:verified", corrected):
        raise ValueError("Managed recovery refused: expected exactly two bool ABI adaptations")
    return count


def git(repository: Path, *arguments: str) -> str:
    """Inspect committed bytes without changing refs or the working tree."""
    return subprocess.run(["git", "-C", str(repository), *arguments], check=True, capture_output=True).stdout.decode(
        "utf-8"
    )


def stage(repository: Path, recovery_sha: str) -> dict[str, str | int]:
    """Copy only the validated managed wrapper, retaining all original native source."""
    if not re.fullmatch(r"[0-9a-f]{40}", recovery_sha):
        raise ValueError("Managed recovery refused: an immutable full commit SHA is required")
    if git(repository, "status", "--porcelain", "--untracked-files=no"):
        raise ValueError("Managed recovery refused: release source is dirty")
    original = git(repository, "show", f"HEAD:{CONVERTER}")
    corrected = git(repository, "show", f"{recovery_sha}:{CONVERTER}")
    original_mode = git(repository, "ls-tree", "HEAD", "--", CONVERTER).split()[:2]
    corrected_mode = git(repository, "ls-tree", recovery_sha, "--", CONVERTER).split()[:2]
    if original_mode != ["100644", "blob"] or original_mode != corrected_mode:
        raise ValueError("Managed recovery refused: wrapper mode or object type changed")
    count = verify_adaptation(original, corrected)
    (repository / CONVERTER).write_bytes(corrected.encode("utf-8"))
    return {
        "original_sha": git(repository, "rev-parse", "HEAD").strip(),
        "recovery_sha": recovery_sha,
        "adapted_calls": count,
        "wrapper_sha256": hashlib.sha256(corrected.encode("utf-8")).hexdigest(),
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("recovery_sha")
    arguments = parser.parse_args()
    print(json.dumps(stage(Path.cwd(), arguments.recovery_sha), sort_keys=True))
