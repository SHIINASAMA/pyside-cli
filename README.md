# CLI for PySide Template

## Quick Overview

This is a companion CLI for **pyside\_template** (not an official PySide tool).

It helps you quickly create a template project:

```bash
mkdir app && cd app
pip install pyside-cli>=1.0.0
pyside-cli create .           # requires: git
```

You can also build the project or run tests with a single command.

```bash
pyside-cli build --onefile # for build: requires pyside6, nuitka
pyside-cli test            # for testing: requires pytest
```

## Links

-   [PyPI - pyside-cli](https://pypi.org/project/pyside-cli/)
    
-   [pyside\_template](https://github.com/SHIINASAMA/pyside_template)
