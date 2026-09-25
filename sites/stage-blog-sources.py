#!/usr/bin/env python3
"""Stage images required by published Blog notes, leaving other Vault files alone."""

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
    allowed_assets = set()
    for article in manifest["articles"]:
        name = article["file"]
        if not name.startswith("raw/my-vault/Note/"):
            continue
        source = ROOT / name
        if not source.is_file() or hashlib.sha256(source.read_bytes()).hexdigest() != article["sha256"]:
            raise SystemExit(f"Missing or changed source (rebuild first): {name}")
        for match in EMBEDS.finditer(source.read_text(encoding="utf-8")):
            reference = match.group(1).split("|", 1)[0].split("#", 1)[0]
            asset = VAULT / reference
            if not asset.is_file():
                raise SystemExit(f"Missing asset: {asset}")
            allowed_assets.add(str(asset.relative_to(ROOT)))
    tracked = subprocess.check_output(
        ["git", "ls-files", "-z", "--", "raw/my-vault/Assets"], cwd=ROOT
    ).decode().split("\0")
    stale = sorted(set(filter(None, tracked)) - allowed_assets)
    if stale:
        subprocess.run(["git", "rm", "--cached", "--", *stale], cwd=ROOT, check=True)
    if allowed_assets:
        subprocess.run(["git", "add", "-f", "--", *sorted(allowed_assets)], cwd=ROOT, check=True)
    print(f"Staged {len(allowed_assets)} Blog assets; removed {len(stale)} stale index entries")


if __name__ == "__main__":
    main()
