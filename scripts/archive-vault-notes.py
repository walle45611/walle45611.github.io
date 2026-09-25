#!/usr/bin/env python3
"""Archive My vault notes without changing the source mirror or existing summaries."""

from __future__ import annotations

import argparse
import hashlib
import re
from datetime import date
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
VAULT_ROOT = ROOT / "raw/my-vault"
NOTE_ROOT = ROOT / "raw/my-vault/Note"
DASH_ROOT = ROOT / "raw/my-vault/00_Dashboard"
CLIP_ROOT = ROOT / "raw/web-clipper"
SUMMARIES = ROOT / "wiki/summaries"
ARCHIVES = ROOT / "wiki/archives"
INDEX = ROOT / "wiki/index.md"


def source_of(path: Path) -> str | None:
    match = re.search(r"(?m)^- source: `([^`]+)`", path.read_text(errors="replace"))
    return match.group(1) if match else None


def catalog() -> dict[str, Path]:
    result = {}
    for path in sorted(SUMMARIES.glob("*.md")):
        source = source_of(path)
        if source and source not in result:
            result[source] = path
    return result


def clean(line: str) -> str:
    line = re.sub(r"!\[\[[^]]+\]\]", "", line)
    line = re.sub(r"\[\[([^]|]+)\|([^]]+)\]\]", r"\2", line)
    line = re.sub(r"\[\[([^]]+)\]\]", r"\1", line)
    line = re.sub(r"!?\[([^]]+)\]\([^)]+\)", r"\1", line)
    line = re.sub(r"^(?:[-*+] |\d+[.)] |\*\*\d+[.)]\*\* )", "", line.strip())
    line = re.sub(r"<[^>]+>", "", line)
    line = line.replace("**", "").replace("`", "")
    return re.sub(r"\s+", " ", line).strip(" -|：:")


def source_outline(path: Path) -> tuple[str, str, list[str]]:
    raw = path.read_text(errors="replace")
    body = re.sub(r"\A---\s*\n.*?\n---\s*\n", "", raw, flags=re.S)
    yaml_title = re.search(r'(?m)^title:\s*["\']?(.+?)["\']?\s*$', raw[:2000])
    first_line = next((line.strip() for line in body.splitlines() if line.strip()), "")
    heading = re.match(r"^#\s+(.+)$", first_line)
    title = clean(heading.group(1)) if heading else path.stem
    if yaml_title and not heading:
        title = yaml_title.group(1).strip('"\' ')
    headings: list[str] = []
    prose: list[str] = []
    in_code = False
    for line in body.splitlines():
        stripped = line.strip()
        if stripped.startswith(("```", "~~~")):
            in_code = not in_code
            continue
        if in_code or not stripped:
            continue
        h = re.match(r"^#{2,4}\s+(.+)", stripped)
        if h:
            value = clean(h.group(1))
            if value and value not in headings and not re.match(r"^(初讀摘要|概述|參考|References?|圖片|Figure Sources)$", value, re.I):
                headings.append(value)
            continue
        if stripped.startswith(("#", "|", "![", "![[", "$$", "\\", ">")):
            continue
        if "回到 [[" in stripped or stripped in ("---", "***"):
            continue
        value = clean(stripped)
        if len(value) < 25 or value.startswith(("http://", "https://")):
            continue
        if re.search(r"(?:password|secret|token|api.?key)\s*[:=]", value, re.I):
            continue
        if len(value) > 240:
            value = value[:237].rsplit(" ", 1)[0] + "…"
        if value not in prose:
            prose.append(value)
    return title, "、".join(headings[:6]), prose[:3]


def slug(path: Path) -> str:
    part = path.relative_to(VAULT_ROOT).parts[0].lower()
    if part == "00_dashboard":
        part = "dashboard"
    else:
        part = path.relative_to(NOTE_ROOT).parts[0].lower()
    words = re.findall(r"[a-z0-9]+", path.stem.lower())
    readable = "-".join(words[:5])[:65].strip("-") or "note"
    digest = hashlib.sha256(path.relative_to(ROOT).as_posix().encode()).hexdigest()[:9]
    return f"my-vault-{part}-{readable}-{digest}"


def archive_link(path: Path) -> str:
    part = "dashboard" if path.is_relative_to(DASH_ROOT) else path.relative_to(NOTE_ROOT).parts[0].lower()
    return f"../archives/my-vault-{part}.md"


def new_summary(path: Path, today: str, refresh: bool = False) -> tuple[Path, str]:
    source = path.relative_to(ROOT).as_posix()
    title, headings, prose = source_outline(path)
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    target = SUMMARIES / f"{slug(path)}.md"
    if target.exists() and not refresh:
        raise FileExistsError(target)
    if target.exists() and source_of(target) != source:
        raise FileExistsError(f"slug collision: {target}")
    section = "Dashboard" if path.is_relative_to(DASH_ROOT) else path.relative_to(NOTE_ROOT).parts[0]
    topic = f"原筆記涵蓋 {headings}。" if headings else (prose[0] if prose else f"原筆記記錄 {title} 相關內容。")
    if not headings and not prose:
        raw_body = re.sub(r"\A---\s*\n.*?\n---\s*\n", "", path.read_text(errors="replace"), flags=re.S)
        if not raw_body.strip():
            topic = "來源筆記目前沒有正文，尚無內容可摘要。"
        elif "![[" in raw_body or "![" in raw_body:
            topic = "來源筆記主要保存圖片嵌入；文字不足以獨立摘要，需查看原始圖片。"
        elif "```" in raw_body or "~~~" in raw_body:
            topic = f"來源筆記以 {title} 的命令或程式碼範例為主，使用情境與結果仍需依原文確認。"
        elif "|" in raw_body:
            topic = f"來源筆記以表格整理 {title}，細節請查看原表格。"
    if len(topic) > 210:
        topic = topic[:207] + "…"
    description = f"整理「{title}」的原筆記內容與章節。"
    lines = [
        f"# {title}", "", f"- source: `{source}`", f"- source_sha256: `{digest}`",
        f"- ingested_at: {today}", "- type: my-vault note summary",
        f"- collection: {section}", "", "## Summary", "", topic,
        "", "## Source Notes", "",
    ]
    details = prose if headings else prose[1:]
    if details:
        lines.extend(f"- {item}" for item in details)
    else:
        lines.append("- 其餘細節請直接查原文；此頁只整理已讀到的文字、章節與內容形式。")
    lines.extend(["", "## Navigation", "", f"- [回到 {section} 歸檔](<{archive_link(path)}>)", ""])
    target.write_text("\n".join(lines))
    return target, description


def archive_page(name: str, entries: list[tuple[Path, Path]]) -> None:
    target = ARCHIVES / f"{name}.md"
    title = {"my-vault-dashboard": "My vault：主題總覽", "my-vault-research": "My vault：Research 筆記", "my-vault-tech": "My vault：Tech 筆記", "web-clipper": "Web Clipper 來源"}[name]
    lines = [f"# {title}", "", "此頁依來源路徑列出已歸檔的筆記與摘要。完整原文仍以 `raw/` 檔案為準。", ""]
    groups: dict[str, list[tuple[Path, Path]]] = {}
    for source, summary in entries:
        parts = source.relative_to(ROOT / "raw").parts
        group = "/".join(parts[2:-1]) if parts[0] == "my-vault" else "剪藏"
        if parts[0] == "my-vault" and parts[1] == "00_Dashboard":
            group = "主題總覽"
        groups.setdefault(group or "一般筆記", []).append((source, summary))
    for group, items in sorted(groups.items()):
        lines.extend([f"## {group}", ""])
        for source, summary in sorted(items, key=lambda item: item[0].name.casefold()):
            rel = "../summaries/" + summary.name
            lines.append(f"- [{source.stem}](<{rel}>) · `{source.relative_to(ROOT).as_posix()}`")
        lines.append("")
    target.write_text("\n".join(lines))


def update_index(new: list[tuple[Path, str]], today: str) -> None:
    data = INDEX.read_text()
    if "## Archives" not in data:
        archive_rows = (
            "## Archives\n\n"
            "- [My vault：主題總覽](./archives/my-vault-dashboard.md)\n"
            "- [My vault：Research 筆記](./archives/my-vault-research.md)\n"
            "- [My vault：Tech 筆記](./archives/my-vault-tech.md)\n"
            "- [Web Clipper 來源](./archives/web-clipper.md)\n\n"
        )
        data = data.replace("## Concepts\n", archive_rows + "## Concepts\n", 1)
    dashboard_row = "- [My vault：主題總覽](./archives/my-vault-dashboard.md)"
    if dashboard_row not in data:
        data = data.replace("## Archives\n", "## Archives\n\n" + dashboard_row + "\n", 1)
    rows = []
    for path, desc in new:
        row = f"- [{path.stem}](./summaries/{path.name}) · {today}: {desc}"
        pattern = rf"(?m)^- \[{re.escape(path.stem)}\]\(\./summaries/{re.escape(path.name)}\).*?$"
        if re.search(pattern, data):
            data = re.sub(pattern, lambda _: row, data, count=1)
        else:
            rows.append(row)
    if rows:
        data = data.replace("## Summaries\n", "## Summaries\n\n" + "\n".join(rows) + "\n", 1)
    INDEX.write_text(data)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="report missing and changed sources without writing")
    parser.add_argument("--refresh-generated", action="store_true", help="refresh only summaries created by this script")
    args = parser.parse_args()
    existing = catalog()
    notes = sorted(NOTE_ROOT.rglob("*.md")) + sorted(DASH_ROOT.rglob("*.md"))
    clips = sorted(CLIP_ROOT.rglob("*.md"))
    sources = notes + clips
    missing = [path for path in sources if path.relative_to(ROOT).as_posix() not in existing]
    changed = []
    for source, summary in existing.items():
        path = ROOT / source
        if not path.is_file():
            changed.append((source, "source missing"))
            continue
        match = re.search(r"(?m)^- source_sha256: `([a-f0-9]+)`", summary.read_text(errors="replace"))
        if match and match.group(1) != hashlib.sha256(path.read_bytes()).hexdigest():
            changed.append((source, "source changed"))
    if args.check:
        print(f"sources={len(sources)} archived={len(sources)-len(missing)} missing={len(missing)} changed={len(changed)}")
        for path in missing[:20]:
            print("MISSING", path.relative_to(ROOT))
        for source, reason in changed[:20]:
            print("CHANGED", source, reason)
        raise SystemExit(bool(missing or changed))
    today = date.today().isoformat()
    new = []
    generate = missing
    if args.refresh_generated:
        generate = [path for path in notes if path in missing or "- type: my-vault note summary" in existing[path.relative_to(ROOT).as_posix()].read_text(errors="replace")]
    for path in generate:
        if not path.is_relative_to(VAULT_ROOT):
            raise RuntimeError(f"Web clipper source lacks an existing summary: {path}")
        target, desc = new_summary(path, today, refresh=args.refresh_generated)
        existing[path.relative_to(ROOT).as_posix()] = target
        new.append((target, desc))
    ARCHIVES.mkdir(exist_ok=True)
    for name, prefix in (("my-vault-dashboard", "raw/my-vault/00_Dashboard/"), ("my-vault-research", "raw/my-vault/Note/Research/"), ("my-vault-tech", "raw/my-vault/Note/Tech/"), ("web-clipper", "raw/web-clipper/")):
        items = [(ROOT / source, summary) for source, summary in existing.items() if source.startswith(prefix)]
        archive_page(name, items)
    update_index(new, today)
    print(f"sources={len(sources)} previously_archived={len(sources)-len(missing)} written={len(new)} changed={len(changed)}")


if __name__ == "__main__":
    main()
