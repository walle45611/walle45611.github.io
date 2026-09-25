#!/usr/bin/env python3
"""Build temporary blog pages from tagged notes in raw/my-vault."""

import argparse
import hashlib
import html
import json
import re
import shutil
import struct
from pathlib import Path


EMBED = re.compile(r"!?\[\[(.*?)\]\]")
TIKZ = re.compile(r"(?ms)^```tikz[ \t]*\n(.*?)^```[ \t]*$")
HEADING = re.compile(r"^(#{1,5})([ \t]+)(.*)$")
HIGHLIGHT = re.compile(r"==(.+?)==(?![A-Za-z0-9_])")
CALLOUT = re.compile(r"^([ \t]*>[ \t]*)\[!(\w+)\][+-]?[ \t]*(.*)$", re.I)
CALLOUT_NAMES = {
    "note": "筆記", "info": "資訊", "tip": "提示", "warning": "注意",
    "important": "重點", "example": "範例", "question": "問題",
}
DISPLAY_MATH = re.compile(r"(?ms)^\$\$[ \t]*\n(.*?)^\$\$[ \t]*$")


def image_dimensions(content: bytes, suffix: str) -> tuple[int, int]:
    """Read dimensions from the PNG/JPEG formats used by Vault blog images."""
    if suffix.lower() == ".png" and content.startswith(b"\x89PNG\r\n\x1a\n"):
        return struct.unpack(">II", content[16:24])
    if suffix.lower() in {".jpg", ".jpeg"} and content.startswith(b"\xff\xd8"):
        offset = 2
        while offset + 4 < len(content):
            if content[offset] != 0xFF:
                offset += 1
                continue
            marker = content[offset + 1]
            offset += 2
            if marker in {0xD8, 0xD9, 0x01} or 0xD0 <= marker <= 0xD7:
                continue
            length = int.from_bytes(content[offset : offset + 2], "big")
            if marker in {0xC0, 0xC1, 0xC2, 0xC3, 0xC5, 0xC6, 0xC7, 0xC9, 0xCA, 0xCB, 0xCD, 0xCE, 0xCF}:
                height = int.from_bytes(content[offset + 3 : offset + 5], "big")
                width = int.from_bytes(content[offset + 5 : offset + 7], "big")
                return width, height
            offset += length
    raise ValueError(f"Unsupported or invalid blog image: {suffix}")


def split_blog_properties(note: str, source: Path) -> tuple[dict[str, str], str]:
    """Read the vault's publication settings without exporting its properties."""
    match = re.match(r"\A---\r?\n(.*?)\r?\n---\r?\n", note, re.S)
    if not match:
        raise ValueError(f"Missing blog properties in source note: {source}")
    properties = {}
    for line in match.group(1).splitlines():
        key, separator, value = line.partition(":")
        if separator:
            properties[key.strip()] = value.strip()
    url = properties.get("blog_url", "")
    slug_match = re.fullmatch(r"https://blog\.walle4561\.com/articles/posts/([^/\s]+)/", url)
    date = properties.get("blog_date", "")
    if properties.get("blog") != "true" or not slug_match or not re.fullmatch(r"\d{4}-\d{2}-\d{2}", date):
        raise ValueError(f"Invalid blog properties in source note: {source}")
    try:
        title = json.loads(properties["blog_title"])
    except (KeyError, json.JSONDecodeError) as error:
        raise ValueError(f"Invalid blog title in source note: {source}") from error
    if not isinstance(title, str) or not title.strip():
        raise ValueError(f"Empty blog title in source note: {source}")
    body = note[match.end():].strip()
    if not body:
        raise ValueError(f"Empty blog body in source note: {source}")
    topic = properties.get("blog_topic", "")
    if topic and topic not in {"data-structures", "algorithms", "problem-solving"}:
        raise ValueError(f"Invalid blog topic in source note: {source}")
    return {"title": title, "slug": slug_match.group(1), "date": date, "topic": topic}, body


def normalize_display_math(body: str) -> str:
    """Put multiline display delimiters on their own lines for the renderer."""
    output = []
    in_math = False
    fenced = False
    fence_char = ""
    for line in body.splitlines():
        fence = re.match(r"^[ \t]*(`{3,}|~{3,})", line)
        if fence:
            if not fenced:
                fenced, fence_char = True, fence.group(1)[0]
            elif fence.group(1)[0] == fence_char:
                fenced = False
            output.append(line)
            continue
        if fenced:
            output.append(line)
            continue

        marker = line.find("$$")
        if marker < 0 or line[:marker].count("`") % 2:
            output.append(line)
            continue
        if not in_math and line.find("$$", marker + 2) >= 0:
            output.append(line)  # A complete display expression on one line.
            continue
        before, after = line[:marker], line[marker + 2:]
        if before.strip():
            output.append(before.rstrip())
        output.append("$$")
        if after.strip():
            output.append(after.lstrip() if not in_math else after.rstrip())
        in_math = not in_math

    body = "\n".join(output)

    def compact(match: re.Match[str]) -> str:
        lines = [line.rstrip() for line in match.group(1).splitlines() if line.strip()]
        return "$$\n" + "\n".join(lines) + "\n$$"

    return DISPLAY_MATH.sub(compact, body)


def convert_obsidian_format(body: str) -> str:
    """Keep vault content intact while adapting Obsidian syntax for the blog."""
    output = []
    fenced = False
    fence_char = ""
    fence_width = 0
    for line in body.splitlines():
        fence = re.match(r"^[ \t]*(`{3,}|~{3,})", line)
        if fence and not fenced:
            fenced = True
            fence_char = fence.group(1)[0]
            fence_width = len(fence.group(1))
            output.append(line)
            continue
        if fenced:
            output.append(line)
            if fence and fence.group(1)[0] == fence_char and len(fence.group(1)) >= fence_width:
                fenced = False
            continue
        if "<img " in line:
            if output and output[-1].strip():
                output.append("")
            output.extend((line, ""))
            continue

        heading = HEADING.match(line)
        if heading:
            line = "#" + line  # The article title is already the page's H1.
        callout = CALLOUT.match(line)
        if callout:
            label = callout.group(3) or CALLOUT_NAMES.get(callout.group(2).lower(), callout.group(2))
            line = f"{callout.group(1)}**{label}**"
        def replace_highlight(match: re.Match[str]) -> str:
            before = line[:match.start()]
            end = line[:match.end()]
            if before.count("`") % 2 or end.count("`") % 2:
                return match.group(0)
            if before.count("$") % 2 or end.count("$") % 2:
                return match.group(0)
            return f"<mark>{match.group(1)}</mark>"

        line = HIGHLIGHT.sub(replace_highlight, line)
        if line.startswith("![") and output and output[-1].strip():
            output.append("")
        output.append(line)
        if line.startswith("!["):
            output.append("")
    return "\n".join(output).strip()


def export(vault: Path, build_dir: Path) -> None:
    dashboard = vault / "00_Dashboard" / "資料結構和演算法 Overview.md"
    overview = dashboard.read_text(encoding="utf-8")
    def link_targets(content: str) -> list[str]:
        return [name.split("|", 1)[0].split("#", 1)[0] for name in re.findall(r"\[\[([^\]]+)\]\]", content)]

    names = link_targets(overview)
    algorithm_names = set(link_targets(overview.split("# 演算法", 1)[1]))
    if len(names) != len(set(names)):
        raise ValueError("Dashboard has duplicate note links")

    articles = {}
    slugs = set()
    for source in sorted((vault / "Note").rglob("*.md")):
        name = source.stem
        note = source.read_text(encoding="utf-8")
        frontmatter = re.match(r"\A---\r?\n(.*?)\r?\n---\r?\n", note, re.S)
        if not frontmatter or not re.search(r"(?m)^blog:\s*true\s*$", frontmatter.group(1)):
            continue
        properties, body = split_blog_properties(note, source)
        slug = properties["slug"]
        if slug in slugs:
            raise ValueError(f"Duplicate blog slug in source note: {source}")
        slugs.add(slug)
        articles[name] = (source, properties, body)

    posts_dir = build_dir / "vault-posts"
    assets_dir = build_dir / "vault-assets"
    posts_dir.mkdir(parents=True, exist_ok=True)
    assets_dir.mkdir(parents=True, exist_ok=True)
    expected_posts = set()
    expected_assets = set()

    for name in articles:
        source, properties, body = articles[name]

        def replace_embed(match: re.Match[str]) -> str:
            reference = match.group(1).split("|", 1)[0].split("#", 1)[0]
            if not match.group(0).startswith("!"):
                linked_name = reference.removesuffix(".md")
                if linked_name not in articles:
                    raise ValueError(f"Unmapped note link in {name}: {reference}")
                return f"[{linked_name}](/articles/posts/{articles[linked_name][1]['slug']}/)"
            asset = vault / reference
            if not asset.is_file():
                raise FileNotFoundError(f"Missing asset in {name}: {asset}")
            content = asset.read_bytes()
            digest = hashlib.sha256(content).hexdigest()[:20]
            asset_name = f"{digest}{asset.suffix.lower()}"
            target = assets_dir / asset_name
            if not target.exists():
                shutil.copyfile(asset, target)
            expected_assets.add(asset_name)
            width, height = image_dimensions(content, asset.suffix)
            alt = html.escape(asset.stem, quote=True)
            return f'<img src="/vault-assets/{asset_name}" alt="{alt}" width="{width}" height="{height}" decoding="async">'

        body = normalize_display_math(convert_obsidian_format(EMBED.sub(replace_embed, body)))

        if TIKZ.search(body):
            raise ValueError(f"TikZ remains in source note: {source}")
        slug = properties["slug"]
        title = properties["title"]
        topic_section = properties["topic"] or (
            "algorithms" if name in algorithm_names else
            "data-structures" if name in names else None
        )
        description = f"{title}的重點整理。"
        date = properties["date"]
        frontmatter = "".join([
            "---\n",
            f"title: {json.dumps(title, ensure_ascii=False)}\n",
            f"slug: {slug}\n",
            f"topic_section: {topic_section}\n" if topic_section else "",
            f"description: {json.dumps(description, ensure_ascii=False)}\n",
            f"date: {date}\n",
            "blog: true\n",
            f"vault_source: {json.dumps(str(source.relative_to(vault)), ensure_ascii=False)}\n",
            "---\n\n",
        ])
        target = posts_dir / f"{slug}.md"
        target.write_text(frontmatter + body + "\n", encoding="utf-8")
        expected_posts.add(target.name)

    for path in posts_dir.glob("*.md"):
        if path.name not in expected_posts:
            path.unlink()
    for path in assets_dir.iterdir():
        if path.is_file() and path.name not in expected_assets:
            path.unlink()
    print(f"Exported {len(expected_posts)} posts and {len(expected_assets)} assets")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    root = Path(__file__).resolve().parents[1]
    parser.add_argument("--vault", type=Path, default=root / "raw" / "my-vault")
    parser.add_argument("--build-dir", type=Path, default=root / ".build" / "site")
    args = parser.parse_args()
    export(args.vault, args.build_dir)
