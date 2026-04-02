## Why

The post [source/_posts/複雜度計算.md](/Users/heguowei/project/blog/source/_posts/複雜度計算.md) already enables `mathjax: true`, and the site configuration also enables MathJax rendering. The remaining problem is the article content itself: some formulas, especially in the Extended Master Theorem section, mix prose and LaTeX in a way that leaves `log`, superscripts, and grouped expressions partially unrendered.

This change is needed now because the current output is visibly inconsistent and reduces readability in a teaching-oriented article that depends on mathematical notation being displayed cleanly.

## What Changes

- Standardize the math markup inside `source/_posts/複雜度計算.md` so expressions that should be rendered by MathJax are consistently wrapped and grouped.
- Rewrite the Extended Master Theorem subsection so prose and formulas are separated into stable inline or block math instead of mixed half-rendered text.
- Review nearby sections in the same post for the same formatting pattern and apply the same targeted cleanup where needed.
- Verify the rendered article through the normal Hexo build flow.

## Capabilities

### New Capabilities
- `mathjax-post-rendering`: Blog posts that enable `mathjax: true` can express mathematical statements with consistent MathJax-friendly Markdown so formulas render cleanly in the generated site.

### Modified Capabilities
- None.

## Impact

- Affected content: `source/_posts/複雜度計算.md`
- Affected OpenSpec artifacts: `proposal.md`, `design.md`, `tasks.md`, and a new spec for MathJax post rendering behavior
- No new runtime dependencies or site-wide configuration changes
