## Appendix A — Default Configuration

The following `.cmakefmt.toml` shows all options at their default values:

```toml
lineWidth = 80
wrapStyle = "cascade"
firstArgSameLine = true
wrapArgThreshold = 0
indentWidth = 2
indentStyle = "space"
# continuationIndentWidth — inherits indentWidth
# genexIndentWidth — inherits indentWidth
maxBlankLines = 1
minBlankLinesBetweenBlocks = 0
blankLineBetweenSections = false
commandCase = "lower"
keywordCase = "upper"
customKeywords = []
literalCase = "unchanged"
closingParenNewline = true
spaceBeforeParen = false
spaceInsideParen = "remove"
commentPreservation = "preserve"
# commentWidth — inherits lineWidth
alignTrailingComments = false
commentGap = 1
lineEnding = "auto"
finalNewline = true
trimTrailingWhitespace = true
collapseSpaces = true
alignPropertyValues = false
alignConsecutiveSet = false
alignArgGroups = false
genexWrap = "cascade"
genexClosingAngleNewline = true
# perCommandConfig — empty table by default
sortArguments = false
sortKeywordSections = false
disableFormatting = false
ignorePatterns = []
ignoreCommands = []
indentBlockBody = true
endCommandArgs = "remove"
```
