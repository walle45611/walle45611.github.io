#!/usr/bin/env python3
"""Mirror the original Obsidian vault into raw/my-vault."""

import argparse
import json
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TARGET = ROOT / "raw/my-vault"
OBSIDIAN = ROOT / ".obsidian"
DEFAULT_SOURCE = Path.home() / "Library/Mobile Documents/iCloud~md~obsidian/Documents/My vault"


def sync(source: Path) -> None:
    if not source.is_dir() or not (source / ".obsidian").is_dir():
        raise SystemExit(f"Not an Obsidian vault: {source}")
    if source.resolve() == TARGET.resolve():
        raise SystemExit("Source and target must differ")
    TARGET.mkdir(parents=True, exist_ok=True)
    source_names = {item.name for item in source.iterdir()}
    for item in TARGET.iterdir():
        if item.name not in source_names:
            shutil.rmtree(item) if item.is_dir() else item.unlink()
    for item in source.iterdir():
        target = TARGET / item.name
        if item.is_dir():
            subprocess.run(["rsync", "-a", "--delete", f"{item}/", f"{target}/"], check=True)
        elif item.is_file():
            shutil.copy2(item, target)

    for folder in ("plugins", "snippets"):
        source_folder = source / ".obsidian" / folder
        if source_folder.is_dir():
            destination = OBSIDIAN / folder
            destination.mkdir(parents=True, exist_ok=True)
            subprocess.run(["rsync", "-a", f"{source_folder}/", f"{destination}/"], check=True)
    enabled = OBSIDIAN / "community-plugins.json"
    original = source / ".obsidian/community-plugins.json"
    plugins = list(dict.fromkeys(json.loads(enabled.read_text()) + json.loads(original.read_text())))
    enabled.write_text(json.dumps(plugins, ensure_ascii=False, indent=2) + "\n")
    print(f"Mirrored My vault to {TARGET}; enabled plugins: {', '.join(plugins)}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, default=DEFAULT_SOURCE)
    sync(parser.parse_args().source)
