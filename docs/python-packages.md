# Python package support

The python-agent-driver ships CPython 3.12 with the full standard library and 102 explicitly installed top-level pip packages (transitive dependencies are also included).

## Standard library

The complete CPython 3.12 standard library is included. Commonly used modules:

| Module | Description |
|--------|-------------|
| `ast` | Abstract syntax tree |
| `cProfile` | Deterministic profiling |
| `csv` | CSV file reading/writing |
| `datetime` | Date and time types |
| `email` | Email handling |
| `filecmp` | File and directory comparison |
| `fnmatch` | Unix filename pattern matching |
| `glob` | Unix-style pathname expansion |
| `imaplib` | IMAP4 protocol client |
| `json` | JSON encoder/decoder |
| `os` | OS interfaces |
| `pathlib` | Object-oriented filesystem paths |
| `platform` | Platform identification |
| `profile` | Python profiler |
| `re` | Regular expressions |
| `shutil` | High-level file operations |
| `socket` | Low-level networking |
| `ssl` | TLS/SSL wrapper |
| `subprocess` | Subprocess management (shimmed) |
| `tempfile` | Temporary files and directories |

Other standard library modules (`collections`, `itertools`, `functools`, `hashlib`, `struct`, `threading`, `typing`, `urllib`, `xml`, `zipfile`, `tarfile`, `sqlite3`, etc.) are also available.

## Pre-imported packages (zero import cost)

These packages are imported during `pyhl setup` warmup. They are already in `sys.modules` when your code runs, so `import` is instant.

| Package | Import name |
|---------|-------------|
| beautifulsoup4 | `bs4` |
| click | `click` |
| cryptography | `cryptography` |
| Jinja2 | `jinja2` |
| lxml | `lxml` |
| markdown-it-py | `markdown_it` |
| numpy | `numpy` |
| openpyxl | `openpyxl` |
| pandas | `pandas` |
| Pillow | `PIL` |
| pydantic | `pydantic` |
| pypdf | `pypdf` |
| python-dateutil | `dateutil` |
| python-docx | `docx` |
| python-dotenv | `dotenv` |
| python-pptx | `pptx` |
| PyYAML | `yaml` |
| tabulate | `tabulate` |
| tenacity | `tenacity` |
| tqdm | `tqdm` |

## Shipped packages (import cost on first use)

These packages are in the rootfs but not pre-imported. The first `import` pays the usual module load cost.

| Package | Import name |
|---------|-------------|
| aiohttp | `aiohttp` |
| altair | `altair` |
| APScheduler | `apscheduler` |
| bandit | `bandit` |
| bokeh | `bokeh` |
| boto3 | `boto3` |
| builtwith | `builtwith` |
| celery | `celery` |
| chardet | `chardet` |
| charset-normalizer | `charset_normalizer` |
| coverage | `coverage` |
| distro | `distro` |
| docx2txt | `docx2txt` |
| duckdb | `duckdb` |

| exchange-calendars | `exchange_calendars` |
| fabric | `fabric` |
| Faker | `faker` |
| fastapi | `fastapi` |
| feedparser | `feedparser` |
| fpdf2 | `fpdf` |
| gensim | `gensim` |
| gitpython | `git` |
| google-api-python-client | `googleapiclient` |
| hypercorn | `hypercorn` |
| httpx | `httpx` |
| hypothesis | `hypothesis` |
| loguru | `loguru` |
| markdown | `markdown` |
| markdownify | `markdownify` |
| mutagen | `mutagen` |
| networkx | `networkx` |
| nltk | `nltk` |
| numpy-financial | `numpy_financial` |
| odfpy | `odf` |
| paramiko | `paramiko` |
| pdfplumber | `pdfplumber` |
| pdfrw | `pdfrw` |
| pexpect | `pexpect` |
| pipdeptree | `pipdeptree` |
| platformdirs | `platformdirs` |
| plotly | `plotly` |
| polars | `polars` |
| praw | `praw` |
| pycountry | `pycountry` |
| pydub | `pydub` |
| pyflakes | `pyflakes` |
| pygments | `pygments` |
| pylint | `pylint` |
| PyPDF2 | `PyPDF2` |
| pytest | `pytest` |
| pytest-asyncio | `pytest_asyncio` |
| pytest-cov | `pytest_cov` |
| pyxlsb | `pyxlsb` |
| qrcode | `qrcode` |
| radon | `radon` |
| rapidfuzz | `rapidfuzz` |
| rarfile | `rarfile` |
| reportlab | `reportlab` |
| requests | `requests` |
| rope | `rope` |
| ruff | `ruff` |
| schedule | `schedule` |
| scikit-learn | `sklearn` |
| scipy | `scipy` |
| scrapy | `scrapy` |
| send2trash | `send2trash` |
| slack-sdk | `slack_sdk` |
| srt | `srt` |
| statsmodels | `statsmodels` |
| svgwrite | `svgwrite` |
| sympy | `sympy` |
| textblob | `textblob` |
| trafilatura | `trafilatura` |
| tweepy | `tweepy` |
| typer | `typer` |
| typing-extensions | `typing_extensions` |
| uvicorn | `uvicorn` |
| vulture | `vulture` |
| watchdog | `watchdog` |
| websockets | `websockets` |
| wordcloud | `wordcloud` |
| xlrd | `xlrd` |
| xlsxwriter | `xlsxwriter` |
