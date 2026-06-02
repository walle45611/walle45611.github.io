# NLP Evaluation Metrics

## Current View

NLP evaluation metrics are automatic measures used to compare model outputs against expected references or task-specific targets. For generated text, the metric must be chosen carefully because surface overlap, semantic quality, factuality, fluency, and concision are not the same property.

## Stable Conclusions

1. Generated-text tasks often cannot be evaluated cleanly with simple accuracy because there may be many acceptable outputs.
2. ROUGE is a reference-overlap metric commonly used for summarization, especially when comparing generated summaries with human-written reference summaries.
3. Different ROUGE variants measure different kinds of overlap: unigram overlap, bigram overlap, ordered subsequence similarity, or whole-summary sequence similarity.
4. Recall-only evaluation can reward overly long outputs, so precision and F1 should be checked together with recall.
5. Automatic metrics should be treated as proxies. A higher score does not necessarily prove better factuality, readability, or usefulness.

## Working Heuristics

- Start by asking what property the metric actually measures: exact overlap, ordered overlap, semantic similarity, factual consistency, or task success.
- For summarization, report ROUGE-1, ROUGE-2, and ROUGE-L or ROUGE-LSUM together when possible.
- Compare models with the same reference set and preprocessing assumptions; otherwise metric differences may reflect evaluation setup rather than model quality.
- Do not use a single automatic metric as the only decision signal for user-facing generation quality.
- When a workflow includes verification or revision, evaluate both final output quality and the cost of reaching it.

## Related Concepts

- [self-correction-in-language-models](./self-correction-in-language-models.md)
- [harness-engineering](./harness-engineering.md)

## Sources

- [what-is-the-rouge-metric](../summaries/what-is-the-rouge-metric.md)
