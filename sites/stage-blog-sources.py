#!/usr/bin/env python3
"""Stage only published Vault notes and required images for the public Git repo."""

import hashlib
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
VAULT = ROOT / "raw/my-vault"
MANIFEST = ROOT / "sites/content-manifest.json"
EMBEDS = re.compile(r"!\[\[(.*?)\]\]")


def main() -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    allowed = set()
    for article in manifest["articles"]:
        name = article["file"]
        if not name.startswith("raw/my-vault/Note/"):
            continue
        source = ROOT / name
        if not source.is_file() or hashlib.sha256(source.read_bytes()).hexdigest() != article["sha256"]:
            raise SystemExit(f"Missing or changed source (rebuild first): {name}")
        allowed.add(name)
        for match in EMBEDS.finditer(source.read_text(encoding="utf-8")):
            reference = match.group(1).split("|", 1)[0].split("#", 1)[0]
            asset = VAULT / reference
            if not asset.is_file():
                raise SystemExit(f"Missing asset: {asset}")
            allowed.add(str(asset.relative_to(ROOT)))
    for name in ("raw/my-vault/00_Dashboard/資料結構和演算法 Overview.md",):
        if (ROOT / name).is_file():
            allowed.add(name)
    tracked = subprocess.check_output(
        ["git", "ls-files", "-z", "--", "raw/my-vault"], cwd=ROOT
    ).decode().split("\0")
    stale = sorted(set(filter(None, tracked)) - allowed)
    if stale:
        subprocess.run(["git", "rm", "--cached", "--", *stale], cwd=ROOT, check=True)
    subprocess.run(["git", "add", "-f", "--", *sorted(allowed)], cwd=ROOT, check=True)
    print(f"Staged {len(allowed)} public Vault sources; removed {len(stale)} stale index entries")


if __name__ == "__main__":
    main()
