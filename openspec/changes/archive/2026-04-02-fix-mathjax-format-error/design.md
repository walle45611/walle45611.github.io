## Context

This Hexo blog already supports MathJax rendering through the existing site configuration, and the target post already declares `mathjax: true` in its front matter. The defect is localized to the Markdown content in `source/_posts/複雜度計算.md`, where prose and formulas are mixed in ways that produce visibly inconsistent rendering in the generated page.

The most obvious problem is the Extended Master Theorem subsection. It contains expressions like `nlog n`, `log^k n`, and `n^{\log_b a}` split across prose and math boundaries, which makes the output appear partially typeset and partially plain text.

## Goals / Non-Goals

**Goals:**
- Make the Extended Master Theorem subsection render cleanly with MathJax in the generated blog page.
- Normalize nearby math expressions in the same post when they show the same formatting problem.
- Keep the article meaning unchanged while improving formula readability and consistency.

**Non-Goals:**
- Do not change site-wide Hexo, NexT, or MathJax configuration.
- Do not rewrite unrelated sections of the article for style only.
- Do not alter the mathematical claims or expand the content beyond the existing scope.

## Decisions

### Use content-only fixes instead of configuration changes
The repository already has `hexo-math` installed, NexT MathJax enabled, and the post front matter opts into MathJax. Because the defect is visible only in certain formulas within a single post, changing global configuration would be unnecessary and risk unrelated rendering regressions.

Alternative considered: adjusting site or renderer configuration. Rejected because the current issue is caused by inconsistent article markup, not missing runtime support.

### Separate prose from formulas using stable inline and block math
Expressions that define theorem conditions or conclusions will be moved into standalone math blocks when they span an entire statement. Short symbols embedded in sentences will remain inline math.

Alternative considered: keeping the current prose layout and only wrapping missing fragments. Rejected because the current layout is the source of the mixed rendering and would remain fragile.

### Keep notation consistent within the corrected subsection
The corrected subsection will use explicit LaTeX notation such as `\Theta`, `\log`, grouped exponents, and `\bigl(...\bigr)` where needed so MathJax renders the theorem statements uniformly.

Alternative considered: preserving each original notation variant exactly. Rejected because inconsistent notation is part of the readability problem being fixed.

## Risks / Trade-offs

- [Risk] Small content edits could unintentionally change mathematical meaning. → Mitigation: restrict changes to markup structure and preserve the existing statements and examples.
- [Risk] Reformatting display math may affect surrounding paragraph flow. → Mitigation: build the site and review the rendered output for the target section.
- [Risk] Nearby formulas in the same article may have similar issues and be missed. → Mitigation: perform a targeted scan of adjacent math-heavy sections while editing.
