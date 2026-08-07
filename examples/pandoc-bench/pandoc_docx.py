"""Markdown-to-DOCX benchmark — pure Python, no external binaries.

Creates a DOCX file from Markdown using only the standard library
(zipfile + xml.etree.ElementTree). Exercises string processing,
XML generation, ZIP compression, and file I/O.
"""

import re
import xml.etree.ElementTree as ET
import zipfile
from io import BytesIO
from pathlib import Path

MARKDOWN = """\
# Snapshot Benchmark Report

## Executive Summary

This benchmark measures **snapshot restore performance** across multiple
containment backends. The results demonstrate that *optimized snapshots*
with stripped-down root filesystems significantly reduce both memory
footprint and startup latency.

## Methodology

The benchmark harness performs the following steps:

1. **Capture** a snapshot after the runtime is warm
2. **Restore** from the persisted snapshot (the production path)
3. **Execute** the workload inside the restored micro-VM
4. **Measure** wall-clock time and peak resident set size

Each measurement is repeated 100 times. We report p50, p95, and p99
percentiles to capture tail latency behavior.

## Results

| Backend | p50 Restore (ms) | p50 RSS (MiB) | p99 Restore (ms) |
|---------|------------------:|---------------:|------------------:|
| Hyperlight | 38 | 28 | 45 |
| NVX | 120 | 64 | 155 |
| WSLc | 250 | 128 | 310 |

## Key Findings

- Hyperlight snapshot restore is **3.2x faster** than NVX and **6.6x faster**
  than WSLc at p50.
- Memory footprint scales with the initrd size: a stripped 201 MB initrd
  uses only 28 MiB RSS versus 8 GiB for the full ML stack.
- The **snapshot sparsification** pass (punching zero pages) reduces the
  on-disk snapshot from 1024 MiB to 58 MiB.

## Appendix: Raw Data

Sample data from 10 iterations of the pandoc workload:

| Iteration | Restore (ms) | Call (ms) | Total (ms) | RSS (MiB) |
|----------:|-------------:|----------:|-----------:|----------:|
| 1 | 43.1 | 511.3 | 554.4 | 27.9 |
| 2 | 40.6 | 502.4 | 543.0 | 27.8 |
| 3 | 37.6 | 516.8 | 554.5 | 28.1 |
| 4 | 40.3 | 515.9 | 556.3 | 27.9 |
| 5 | 38.7 | 517.4 | 556.1 | 28.0 |
| 6 | 38.1 | 503.9 | 542.0 | 27.8 |
| 7 | 37.6 | 510.2 | 547.8 | 27.9 |
| 8 | 37.7 | 503.2 | 540.9 | 28.0 |
| 9 | 36.4 | 503.5 | 539.9 | 27.8 |
| 10 | 39.0 | 503.3 | 542.3 | 27.9 |
"""

# ── OOXML constants ──────────────────────────────────────────────────────

CONTENT_TYPES_XML = """\
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml"
    ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"""

RELS_XML = """\
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
    Target="word/document.xml"/>
</Relationships>"""

W = "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
ET.register_namespace("w", W)

def _w(tag):
    return f"{{{W}}}{tag}"


# ── Markdown parser ──────────────────────────────────────────────────────

def parse_markdown(text):
    """Minimal Markdown parser → list of block dicts."""
    blocks = []
    lines = text.split("\n")
    i = 0
    table_rows = []

    def flush_table():
        nonlocal table_rows
        if table_rows:
            blocks.append({"type": "table", "rows": table_rows})
            table_rows = []

    while i < len(lines):
        line = lines[i]

        # Headings
        m = re.match(r"^(#{1,6})\s+(.*)", line)
        if m:
            flush_table()
            blocks.append({"type": "heading", "level": len(m.group(1)), "text": m.group(2)})
            i += 1
            continue

        # Table row
        if "|" in line and line.strip().startswith("|"):
            cells = [c.strip() for c in line.strip().strip("|").split("|")]
            # Skip separator rows (|---|---|)
            if all(re.match(r"^[-:]+$", c) for c in cells):
                i += 1
                continue
            table_rows.append(cells)
            i += 1
            continue

        # Ordered list
        m = re.match(r"^\d+\.\s+(.*)", line)
        if m:
            flush_table()
            blocks.append({"type": "list_item", "ordered": True, "text": m.group(1)})
            i += 1
            continue

        # Unordered list
        m = re.match(r"^[-*]\s+(.*)", line)
        if m:
            flush_table()
            blocks.append({"type": "list_item", "ordered": False, "text": m.group(1)})
            i += 1
            continue

        # Blank line
        if not line.strip():
            flush_table()
            i += 1
            continue

        # Paragraph (collect contiguous lines)
        flush_table()
        para_lines = [line]
        i += 1
        while i < len(lines) and lines[i].strip() and not lines[i].startswith("#") and not lines[i].strip().startswith("|"):
            para_lines.append(lines[i])
            i += 1
        blocks.append({"type": "paragraph", "text": " ".join(para_lines)})

    flush_table()
    return blocks


def parse_inline(text):
    """Parse inline formatting (bold, italic) → list of (text, bold, italic) tuples."""
    runs = []
    pattern = re.compile(r"\*\*(.+?)\*\*|\*(.+?)\*")
    pos = 0
    for m in pattern.finditer(text):
        if m.start() > pos:
            runs.append((text[pos:m.start()], False, False))
        if m.group(1):
            runs.append((m.group(1), True, False))
        elif m.group(2):
            runs.append((m.group(2), False, True))
        pos = m.end()
    if pos < len(text):
        runs.append((text[pos:], False, False))
    return runs


# ── DOCX builder ─────────────────────────────────────────────────────────

def make_run(text, bold=False, italic=False):
    r = ET.SubElement(ET.Element("dummy"), _w("r"))
    if bold or italic:
        rpr = ET.SubElement(r, _w("rPr"))
        if bold:
            ET.SubElement(rpr, _w("b"))
        if italic:
            ET.SubElement(rpr, _w("i"))
    t = ET.SubElement(r, _w("t"))
    t.text = text
    t.set("xml:space", "preserve")
    return r


def make_paragraph(text, style=None):
    p = ET.Element(_w("p"))
    if style:
        ppr = ET.SubElement(p, _w("pPr"))
        pstyle = ET.SubElement(ppr, _w("pStyle"))
        pstyle.set(_w("val"), style)
    for chunk, bold, italic in parse_inline(text):
        r = make_run(chunk, bold, italic)
        p.append(r)
    return p


def make_table(rows):
    tbl = ET.Element(_w("tbl"))
    tbl_pr = ET.SubElement(tbl, _w("tblPr"))
    borders = ET.SubElement(tbl_pr, _w("tblBorders"))
    for side in ("top", "left", "bottom", "right", "insideH", "insideV"):
        b = ET.SubElement(borders, _w(side))
        b.set(_w("val"), "single")
        b.set(_w("sz"), "4")
        b.set(_w("space"), "0")
        b.set(_w("color"), "auto")

    for row_cells in rows:
        tr = ET.SubElement(tbl, _w("tr"))
        for cell_text in row_cells:
            tc = ET.SubElement(tr, _w("tc"))
            p = make_paragraph(cell_text.strip())
            tc.append(p)

    return tbl


def blocks_to_docx(blocks):
    body = ET.Element(_w("body"))
    for block in blocks:
        if block["type"] == "heading":
            level = block["level"]
            style = f"Heading{level}"
            p = make_paragraph(block["text"], style=style)
            body.append(p)
        elif block["type"] == "paragraph":
            p = make_paragraph(block["text"])
            body.append(p)
        elif block["type"] == "list_item":
            p = make_paragraph(block["text"], style="ListParagraph")
            body.append(p)
        elif block["type"] == "table":
            tbl = make_table(block["rows"])
            body.append(tbl)

    doc = ET.Element(_w("document"))
    doc.append(body)
    return doc


def write_docx(doc_element, path):
    buf = BytesIO()
    tree = ET.ElementTree(doc_element)

    with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as zf:
        zf.writestr("[Content_Types].xml", CONTENT_TYPES_XML)
        zf.writestr("_rels/.rels", RELS_XML)

        doc_bytes = BytesIO()
        tree.write(doc_bytes, xml_declaration=True, encoding="UTF-8")
        zf.writestr("word/document.xml", doc_bytes.getvalue())

    Path(path).write_bytes(buf.getvalue())
    return len(buf.getvalue())


# ── Main ─────────────────────────────────────────────────────────────────

output_path = "/tmp/pandoc-benchmark.docx"

blocks = parse_markdown(MARKDOWN)
doc = blocks_to_docx(blocks)
size = write_docx(doc, output_path)

print(f"Generated {size} bytes DOCX with {len(blocks)} blocks", flush=True)
print("PANDOC_DOCX_OK", flush=True)
