#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Append a per-commit benchmark entry into persistent history JSON.",
    )
    parser.add_argument(
        "--entry-file",
        required=True,
        help="Path to normalized per-commit benchmark entry JSON.",
    )
    parser.add_argument(
        "--history-file",
        required=True,
        help="Path to persistent benchmark history JSON.",
    )
    parser.add_argument(
        "--dedupe-by-commit",
        action="store_true",
        default=True,
        help="Replace existing entry for commit_sha before appending (default: enabled).",
    )
    parser.add_argument(
        "--no-dedupe-by-commit",
        dest="dedupe_by_commit",
        action="store_false",
        help="Disable commit_sha dedupe behavior.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def load_history(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"schema_version": 1, "entries": []}

    data = load_json(path)

    if isinstance(data, list):
        # Backwards-compatible upgrade path from plain array format.
        return {"schema_version": 1, "entries": data}

    if not isinstance(data, dict):
        raise ValueError(f"history file must contain an object or array: {path}")

    entries = data.get("entries")
    if not isinstance(entries, list):
        raise ValueError(f"history object must contain an entries array: {path}")

    return data


def validate_entry(entry: Any) -> dict[str, Any]:
    if not isinstance(entry, dict):
        raise ValueError("entry file must contain a JSON object")

    commit_sha = entry.get("commit_sha")
    if not isinstance(commit_sha, str) or not commit_sha:
        raise ValueError("entry is missing non-empty commit_sha")

    return entry


def main() -> int:
    args = parse_args()

    entry_file = Path(args.entry_file)
    history_file = Path(args.history_file)

    entry = validate_entry(load_json(entry_file))
    history = load_history(history_file)

    entries: list[dict[str, Any]] = list(history["entries"])

    if args.dedupe_by_commit:
        commit_sha = entry["commit_sha"]
        entries = [existing for existing in entries if existing.get("commit_sha") != commit_sha]

    entries.append(entry)

    history["entries"] = entries

    history_file.parent.mkdir(parents=True, exist_ok=True)
    with history_file.open("w", encoding="utf-8") as handle:
        json.dump(history, handle, indent=2)
        handle.write("\n")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
