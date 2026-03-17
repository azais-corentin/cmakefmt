## 10 · Generator Expressions

Generator expressions (`$<...>`) are treated as **atomic tokens** by the formatter.

- Generator expressions are never split across lines, regardless of length or nesting depth.
- If a generator expression exceeds `lineWidth`, it remains on one line. The resulting line-width violation is accepted.
- Semicolons inside generator expressions separate list items and are left as-is; they are not treated as argument separators for wrapping purposes.
- Nested generator expressions (`$<$<...>:...>`) are likewise atomic — the entire expression, including all nested `$<...>` constructs, is kept on a single line.

There are no configurable options for generator expression formatting in this section.
