## ADDED Requirements

### Requirement: Math-enabled posts render theorem formulas consistently
When a blog post enables `mathjax: true`, mathematical statements in that post SHALL be written with MathJax-compatible Markdown so displayed formulas render as math instead of mixed prose and raw operator text.

#### Scenario: Extended Master Theorem conditions render as display math
- **WHEN** the generated page for `source/_posts/複雜度計算.md` is viewed at the Extended Master Theorem subsection
- **THEN** the conditions and conclusions for `f(n)` and `T(n)` appear as rendered MathJax formulas with correct superscripts, subscripts, and grouped logarithm expressions

### Requirement: Prose and formulas remain readable in math-heavy sections
Math-heavy instructional sections SHALL separate explanatory prose from full formula statements when inline formatting would cause partial rendering or visually inconsistent symbols.

#### Scenario: Mixed prose does not leave raw log notation in theorem text
- **WHEN** the Extended Master Theorem explanation is rendered
- **THEN** terms such as `\log n`, `\log^k n`, and `n^{\log_b a}` appear in math formatting rather than as partially plain-text fragments embedded in prose
