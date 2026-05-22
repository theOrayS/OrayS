#!/usr/bin/env python3
"""Small offline-compatible axconfig-gen subset used by this repository build."""
import argparse
import ast
import copy
import os
import sys
from pathlib import Path

VERSION = "axconfig-gen 0.2.1"


def strip_comment(line: str) -> str:
    in_str = False
    esc = False
    for i, ch in enumerate(line):
        if esc:
            esc = False
            continue
        if ch == "\\" and in_str:
            esc = True
            continue
        if ch == '"':
            in_str = not in_str
            continue
        if ch == "#" and not in_str:
            return line[:i]
    return line


def split_assignment(line: str):
    in_str = False
    esc = False
    for i, ch in enumerate(line):
        if esc:
            esc = False
            continue
        if ch == "\\" and in_str:
            esc = True
            continue
        if ch == '"':
            in_str = not in_str
            continue
        if ch == "=" and not in_str:
            return line[:i].strip(), line[i + 1 :].strip()
    raise ValueError(f"not an assignment: {line}")


def parse_value(text: str):
    text = text.strip()
    if text == "true":
        return True
    if text == "false":
        return False
    # TOML scalar/array syntax used by this repo is also valid Python literal
    # syntax after comments have been removed: strings, hex/decimal integers,
    # underscores, and nested arrays.
    return ast.literal_eval(text)


def load(path: str) -> dict:
    data = {}
    cur = data
    logical = ""
    bracket_depth = 0
    with open(path, "r", encoding="utf-8") as f:
        for raw in f:
            part = strip_comment(raw).strip()
            if not part:
                continue
            if not logical and part.startswith("[") and part.endswith("]"):
                name = part.strip("[]").strip()
                cur = data.setdefault(name, {})
                continue
            if logical:
                logical += " " + part
            else:
                logical = part
            in_str = False
            esc = False
            bracket_depth = 0
            for ch in logical:
                if esc:
                    esc = False
                    continue
                if ch == "\\" and in_str:
                    esc = True
                    continue
                if ch == '"':
                    in_str = not in_str
                    continue
                if not in_str:
                    if ch == "[":
                        bracket_depth += 1
                    elif ch == "]":
                        bracket_depth -= 1
            if bracket_depth > 0:
                continue
            key, value = split_assignment(logical)
            cur[key] = parse_value(value)
            logical = ""
    if logical:
        key, value = split_assignment(logical)
        cur[key] = parse_value(value)
    return data


def deep_merge(dst: dict, src: dict) -> dict:
    for k, v in src.items():
        if isinstance(v, dict) and isinstance(dst.get(k), dict):
            deep_merge(dst[k], v)
        else:
            dst[k] = copy.deepcopy(v)
    return dst


def set_key(data: dict, dotted: str, value):
    cur = data
    parts = dotted.split(".")
    for p in parts[:-1]:
        cur = cur.setdefault(p, {})
        if not isinstance(cur, dict):
            raise SystemExit(f"cannot assign through non-table key: {dotted}")
    cur[parts[-1]] = value


def get_key(data: dict, dotted: str):
    cur = data
    for p in dotted.split("."):
        if not isinstance(cur, dict) or p not in cur:
            raise KeyError(dotted)
        cur = cur[p]
    return cur


def parse_write(item: str):
    if "=" not in item:
        raise SystemExit(f"invalid -w argument: {item}")
    key, value = item.split("=", 1)
    return key.strip(), parse_value(value.strip())


def fmt_scalar(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, int):
        return hex(v) if v >= 10 else str(v)
    if isinstance(v, str):
        return '"' + v.replace('\\', '\\\\').replace('"', '\\"') + '"'
    if isinstance(v, list):
        return "[" + ", ".join(fmt_scalar(x) for x in v) + "]"
    raise TypeError(f"unsupported value type: {type(v).__name__}")


def type_comment(v):
    if isinstance(v, bool):
        return "bool"
    if isinstance(v, int):
        return "uint"
    if isinstance(v, str):
        return "uint" if v.startswith("0x") or v.startswith("0X") else "str"
    if isinstance(v, list):
        if all(isinstance(x, list) and len(x) == 2 for x in v):
            return "[(uint, uint)]"
        if all(isinstance(x, int) for x in v):
            return "[uint]"
    return ""


def emit_line(k, v):
    c = type_comment(v)
    suffix = f" # {c}" if c else ""
    return f"{k} = {fmt_scalar(v)}{suffix}"


def emit_table(data: dict) -> str:
    lines = []
    for k, v in data.items():
        if not isinstance(v, dict):
            lines.append(emit_line(k, v))
    for name, table in data.items():
        if not isinstance(table, dict):
            continue
        if lines and lines[-1] != "":
            lines.append("")
        lines.append(f"[{name}]")
        for k, v in table.items():
            if isinstance(v, dict):
                raise TypeError(f"nested table deeper than one level is unsupported: {name}.{k}")
            lines.append(emit_line(k, v))
    lines.append("")
    return "\n".join(lines)


def main(argv):
    if "--version" in argv or "-V" in argv:
        print(VERSION)
        return 0
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("inputs", nargs="*")
    parser.add_argument("-r", "--read")
    parser.add_argument("-w", "--write", action="append", default=[])
    parser.add_argument("-o", "--output")
    parser.add_argument("-c", "--check")
    parser.add_argument("-h", "--help", action="store_true")
    ns = parser.parse_args(argv)
    if ns.help:
        print("usage: axconfig-gen <input>... [-w key=value] [-r key] [-o output] [-c old]")
        return 0
    data = {}
    for path in ns.inputs:
        if not path:
            continue
        if not os.path.exists(path):
            raise SystemExit(f"input file not found: {path}")
        deep_merge(data, load(path))
    for item in ns.write:
        key, value = parse_write(item)
        set_key(data, key, value)
    if ns.read:
        try:
            print(fmt_scalar(get_key(data, ns.read)))
        except KeyError:
            return 1
        return 0
    out = emit_table(data)
    if ns.output:
        Path(ns.output).parent.mkdir(parents=True, exist_ok=True)
        Path(ns.output).write_text(out)
    else:
        sys.stdout.write(out)
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
