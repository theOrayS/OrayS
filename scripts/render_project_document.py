#!/usr/bin/env python3
"""Render the OrayS project Markdown as a competition-style PDF."""

from __future__ import annotations

import argparse
import re
import subprocess
import tempfile
from pathlib import Path

from bs4 import BeautifulSoup, NavigableString, Tag
from markdown_it import MarkdownIt


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_SOURCE = ROOT / "docs" / "orays-project-document(2).md"
DEFAULT_OUTPUT = ROOT / "docs" / "orays-project-document(2).pdf"
STYLESHEET = ROOT / "docs" / "orays-project-document.css"


def normalized(text: str) -> str:
    return re.sub(r"\s+", "", text)


def convert_figures(soup: BeautifulSoup) -> None:
    chapter = 0
    sequence = 0
    for node in list(soup.body.descendants):
        if isinstance(node, Tag) and node.name == "h2" and "chapter" in node.get("class", []):
            match = re.search(r"第\s*(\d+)\s*章", node.get_text(" ", strip=True))
            chapter = int(match.group(1)) if match else chapter + 1
            sequence = 0
        if not isinstance(node, Tag) or node.name != "p":
            continue
        meaningful = [child for child in node.children if not (
            isinstance(child, NavigableString) and not child.strip()
        )]
        if len(meaningful) != 1 or not isinstance(meaningful[0], Tag) or meaningful[0].name != "img":
            continue
        sequence += 1
        image = meaningful[0].extract()
        alt = image.get("alt", "系统结构图")
        figure = soup.new_tag("figure")
        caption = soup.new_tag("figcaption")
        caption.string = f"图 {chapter}-{sequence}  {alt}"
        figure.append(image)
        figure.append(caption)
        node.replace_with(figure)


def build_html(
    source: Path,
    pages: dict[str, int] | None = None,
    *,
    include_markers: bool = True,
) -> tuple[str, list[tuple[str, str]]]:
    renderer = MarkdownIt("commonmark", {"html": True}).enable("table")
    fragment = renderer.render(source.read_text(encoding="utf-8"))
    soup = BeautifulSoup(f"<body>{fragment}</body>", "html.parser")

    toc_title = next(
        (heading for heading in soup.find_all("h2") if normalized(heading.get_text()) == "目录"),
        None,
    )
    if toc_title is None:
        raise RuntimeError("source document has no level-2 目录 heading")

    headings: list[tuple[Tag, str, str]] = []
    chapter_index = 0
    section_index = 0
    subsection_index = 0
    for heading in soup.find_all(["h2", "h3", "h4"]):
        if heading is toc_title:
            continue
        if heading.name == "h2":
            chapter_index += 1
            section_index = 0
            subsection_index = 0
            key = f"chapter-{chapter_index}"
            heading["class"] = heading.get("class", []) + ["chapter"]
        elif heading.name == "h3":
            section_index += 1
            subsection_index = 0
            key = f"section-{chapter_index}-{section_index}"
        else:
            subsection_index += 1
            key = f"section-{chapter_index}-{section_index}-{subsection_index}"
        heading["id"] = key
        marker_text = f"ORAYS{key.upper().replace('-', '')}"
        if include_markers:
            marker = soup.new_tag("span")
            marker["class"] = "page-marker"
            marker.string = marker_text
            heading.insert(0, marker)
        headings.append((heading, key, heading.get_text(" ", strip=True).replace(marker_text, "")))

    for sibling in list(toc_title.next_siblings):
        if isinstance(sibling, Tag) and sibling.name == "h2":
            break
        sibling.extract()

    toc_title["class"] = ["toc-title"]
    toc = soup.new_tag("nav")
    toc["class"] = "toc"
    for heading, key, title in headings:
        row = soup.new_tag("div")
        row["class"] = ["toc-row", f"level-{heading.name[1:]}"]
        link = soup.new_tag("a", href=f"#{key}")
        link.string = title
        dots = soup.new_tag("span")
        dots["class"] = "toc-dots"
        page = soup.new_tag("span")
        page["class"] = "toc-page"
        page.string = str((pages or {}).get(key, 0))
        row.extend([link, dots, page])
        toc.append(row)
    toc_title.insert_after(toc)
    toc_section = soup.new_tag("section")
    toc_section["class"] = "toc-section"
    toc_title.wrap(toc_section)
    toc_section.append(toc.extract())

    convert_figures(soup)

    base_uri = source.parent.resolve().as_uri() + "/"
    html = f"""<!doctype html>
<html lang="zh-CN">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>操作系统内核设计 - OrayS</title>
  <base href="{base_uri}">
  <style>{STYLESHEET.read_text(encoding="utf-8")}</style>
</head>
{soup.body}
</html>
"""
    return html, [(key, f"ORAYS{key.upper().replace('-', '')}") for _, key, _ in headings]


def print_pdf(html: str, output: Path, workdir: Path) -> None:
    html_path = workdir / "orays-project-document.html"
    html_path.write_text(html, encoding="utf-8")
    command = [
        "google-chrome",
        "--headless",
        "--no-sandbox",
        "--disable-gpu",
        "--allow-file-access-from-files",
        "--no-pdf-header-footer",
        f"--print-to-pdf={output}",
        html_path.as_uri(),
    ]
    subprocess.run(command, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)


def locate_pages(pdf: Path, markers: list[tuple[str, str]]) -> dict[str, int]:
    result = subprocess.run(
        ["pdftotext", "-layout", str(pdf), "-"],
        check=True,
        stdout=subprocess.PIPE,
    )
    text_pages = result.stdout.decode("utf-8", errors="replace").split("\f")
    physical: dict[str, int] = {}
    for key, marker in markers:
        for page_number, text in enumerate(text_pages, start=1):
            if marker in text.replace(" ", ""):
                physical[key] = page_number
                break
    missing = [key for key, _ in markers if key not in physical]
    if missing:
        raise RuntimeError(f"could not locate heading markers in first-pass PDF: {', '.join(missing)}")
    body_start = physical["chapter-1"]
    return {key: page - body_start + 1 for key, page in physical.items()}


def render(source: Path, output: Path) -> None:
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="orays-document-") as directory:
        workdir = Path(directory)
        probe_pdf = workdir / "probe.pdf"
        html, markers = build_html(source)
        print_pdf(html, probe_pdf, workdir)
        pages = locate_pages(probe_pdf, markers)

        for _ in range(2):
            html, markers = build_html(source, pages)
            print_pdf(html, output, workdir)
            verified = locate_pages(output, markers)
            if verified == pages:
                break
            pages = verified
        else:
            raise RuntimeError("table-of-contents page numbers did not stabilize")

        # The probe markers are useful for page discovery but should not pollute
        # copied text or accessibility output in the delivered PDF.
        clean_html, _ = build_html(source, pages, include_markers=False)
        print_pdf(clean_html, output, workdir)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", nargs="?", type=Path, default=DEFAULT_SOURCE)
    parser.add_argument("output", nargs="?", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()
    render(args.source.resolve(), args.output.resolve())


if __name__ == "__main__":
    main()
