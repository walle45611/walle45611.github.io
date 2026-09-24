#!/usr/bin/env python3
"""Export My vault's data-structures and algorithms notes as blog snapshots."""

import argparse
import hashlib
import json
import re
import shutil
from pathlib import Path


SLUGS = {
    "複雜度計算": "複雜度計算",
    "Array 記憶體位址計算全攻略": "array-memory-address",
    "Link List": "linked-list",
    "堆疊 (Stack)": "stack-1",
    "樹（Tree）": "tree-introduc",
    "二元樹 (Binary Tree)": "binary-trees-1",
    "二元搜尋樹 (Binary Search Tree)": "binary-search-tree",
    "Adelson-Velsky and Landis tree(AVL Tree)": "avl-tree",
    "Red-Black tree": "red-black-tree",
    "引線二元樹 (Threaded Binary Tree)": "threaded-binary-tree",
    "延伸二元樹 (Extended Binary Tree)": "extended-binary-tree",
    "M-Way(Degree) Search tree And B-Tree": "m-way-search-tree-and-b-tree",
    "Splay Trees": "splay-trees",
    "堆積 (Heap)": "heap",
    "最小最大堆積 (Min-Max Heap)": "min-max-heap",
    "對稱最小最大堆積 (Symmetric Min-Max Heap，SMMH)": "symmetric-min-max-heap",
    "雙端堆或是雙端優先隊列 (Double-ended Heap OR  double-ended priority queue，DEPQ)": "double-ended-priority-queue",
    "Leftist heap or min Leftist Tree": "leftist-heap",
    "Binomial Tree 和 Binomial Heap 和 Fibonacci Heaps": "binomial-and-fibonacci-heaps",
    "互斥集合 (Disjoint Sets)": "disjoint-sets",
    "Sorting Algo": "sorting-algorithms",
    "Linear-time sorting algo": "linear-time-sorting",
    "Linear Search and Binary Search": "linear-and-binary-search",
    "Graph 基本定義和 DFS 還有 BFS": "graph-dfs-bfs",
    "Activity Network": "activity-network",
    "Hashing": "hashing",
    "Selection Problem": "selection-problem",
    "01 背包問題 (01 Knapsack Problem)": "zero-one-knapsack",
    "LCS vs. Minimum Edit Distance": "lcs-and-edit-distance",
    "Matrix-chain Multiplication": "matrix-chain-multiplication",
    "Rod-Cutting Problem": "rod-cutting",
    "OBST (Optimal Binary Search Tree)": "optimal-binary-search-tree",
    "Spanning Tree": "spanning-tree",
    "Graph Connectivity": "graph-connectivity",
    "Single-Source Shortest Paths Problem": "single-source-shortest-paths",
    "All-Pair Shortest Path Problem": "all-pairs-shortest-paths",
    "Complexity Classes and P vs NP": "complexity-classes-p-vs-np",
    "Reduction & Problem Types": "reductions-and-problem-types",
    "Proving NP-Completeness": "proving-np-completeness",
    "NP-C常見的問題": "common-np-complete-problems",
    "K-means Clustering": "k-means-clustering",
}

LEGACY_DATES = {
    "複雜度計算": "2025-06-23",
    "堆疊 (Stack)": "2024-08-28",
    "樹（Tree）": "2024-08-26",
    "二元樹 (Binary Tree)": "2024-08-26",
}

TITLES = {
    "Array 記憶體位址計算全攻略": "陣列記憶體位址計算",
    "Link List": "鏈結串列（Linked List）",
    "Adelson-Velsky and Landis tree(AVL Tree)": "AVL 樹：平衡條件與旋轉",
    "M-Way(Degree) Search tree And B-Tree": "多路搜尋樹與 B-Tree",
    "Graph 基本定義和 DFS 還有 BFS": "圖論基礎：DFS 與 BFS",
    "Binomial Tree 和 Binomial Heap 和 Fibonacci Heaps": "二項樹、二項堆與費波那契堆",
    "Leftist heap or min Leftist Tree": "左偏堆（Leftist Heap）",
    "Sorting Algo": "排序演算法",
    "Linear-time sorting algo": "線性時間排序演算法",
    "Activity Network": "活動網路（Activity Network）",
    "01 背包問題 (01 Knapsack Problem)": "0/1 背包問題",
    "Single-Source Shortest Paths Problem": "單一起點最短路徑",
    "All-Pair Shortest Path Problem": "所有點對最短路徑",
    "Complexity Classes and P vs NP": "複雜度類別：P 與 NP",
    "NP-C常見的問題": "常見 NP 完全問題",
}

EMBED = re.compile(r"!?\[\[(.*?)\]\]")
TIKZ = re.compile(r"(?ms)^```tikz[ \t]*\n(.*?)^```[ \t]*$")


def export(vault: Path, site: Path) -> None:
    dashboard = vault / "00_Dashboard" / "資料結構和演算法 Overview.md"
    overview = dashboard.read_text(encoding="utf-8")
    names = re.findall(r"\[\[([^\]]+)\]\]", overview)
    algorithm_names = set(re.findall(r"\[\[([^\]]+)\]\]", overview.split("# 演算法", 1)[1]))
    if set(names) != set(SLUGS) or len(names) != len(SLUGS):
        raise ValueError("Dashboard links differ from the export map")

    posts_dir = site / "vault-posts"
    assets_dir = site / "public" / "vault-assets"
    posts_dir.mkdir(parents=True, exist_ok=True)
    assets_dir.mkdir(parents=True, exist_ok=True)
    expected_posts = set()
    expected_assets = set()

    for name in names:
        source = vault / "Note" / "Research" / f"{name}.md"
        body = source.read_text(encoding="utf-8").strip()
        if not body:
            if name == "Rod-Cutting Problem":
                print(f"Skipped empty source note: {source}")
                continue
            raise ValueError(f"Empty source note: {source}")

        def replace_embed(match: re.Match[str]) -> str:
            reference = match.group(1).split("|", 1)[0].split("#", 1)[0]
            if not match.group(0).startswith("!"):
                linked_name = reference.removesuffix(".md")
                if linked_name not in SLUGS:
                    raise ValueError(f"Unmapped note link in {name}: {reference}")
                return f"[{linked_name}](/articles/posts/{SLUGS[linked_name]}/)"
            asset = vault / reference
            if not asset.is_file():
                raise FileNotFoundError(f"Missing asset in {name}: {asset}")
            digest = hashlib.sha256(asset.read_bytes()).hexdigest()[:20]
            asset_name = f"{digest}{asset.suffix.lower()}"
            target = assets_dir / asset_name
            if not target.exists():
                shutil.copyfile(asset, target)
            expected_assets.add(asset_name)
            alt = asset.stem.replace("[", r"\[").replace("]", r"\]")
            return f"![{alt}](/vault-assets/{asset_name})"

        body = EMBED.sub(replace_embed, body)

        if TIKZ.search(body):
            raise ValueError(f"TikZ remains in source note: {source}")
        slug = SLUGS[name]
        title = TITLES.get(name, name)
        category = "演算法" if name in algorithm_names else "資料結構"
        topic_section = "algorithms" if name in algorithm_names else "data-structures"
        description = f"My vault {category}筆記：{title}。"
        date = LEGACY_DATES.get(name, "2026-09-24")
        frontmatter = (
            "---\n"
            f"title: {json.dumps(title, ensure_ascii=False)}\n"
            f"slug: {slug}\n"
            f"topic_section: {topic_section}\n"
            f"description: {json.dumps(description, ensure_ascii=False)}\n"
            f"date: {date}\n"
            "blog: true\n"
            f"vault_source: {json.dumps(str(source.relative_to(vault)), ensure_ascii=False)}\n"
            "---\n\n"
        )
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
    parser.add_argument("--vault", type=Path, required=True)
    parser.add_argument("--site", type=Path, default=Path(__file__).resolve().parent)
    args = parser.parse_args()
    export(args.vault, args.site)
