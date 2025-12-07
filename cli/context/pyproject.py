import sys
from pathlib import Path

import glom
import toml


def _resolve_package_path(entry_point: str) -> str:
    """
    Resolve the package path from an entry point string.
    For example: "cli.__main__:main" -> "cli"

    It finds the deepest package directory (containing __init__.py)
    in the module path before the function specification.
    """
    # Split the entry point to get the module path (before the colon)
    module_path = entry_point.split(':')[0]
    parts = module_path.split('.')

    # Find the deepest package that exists (contains __init__.py)
    for i in range(len(parts), 0, -1):
        package_name = '.'.join(parts[:i])
        package_dir = Path(package_name.replace('.', '/'))
        init_file = package_dir / '__init__.py'

        if init_file.exists():
            return package_name

    # If no package with __init__.py found, return the first part
    return parts[0] if parts else ""


class PyProjectConfig:
    extra_nuitka_options_list = []
    extra_pyinstaller_options_list = []
    lang_list = []
    scripts = {}

    def __init__(self):
        """get and build vars from pyproject.toml
         1. nuitka command options list
         2. pyinstaller command options list
         3. enabled languages list
         4. project.scripts entries with resolved package paths"""
        with open("pyproject.toml") as f:
            data = toml.load(f)
        nuitka_config = glom.glom(data, "tool.pyside-cli", default={})
        nuitka_platform_config = glom.glom(data, f"tool.pyside-cli.{sys.platform}", default={})
        nuitka_config.update(nuitka_platform_config)

        for k, v in nuitka_config.items():
            if isinstance(v, list) and v:
                cmd = f"--{k}={','.join(v)}"
                self.extra_nuitka_options_list.append(cmd)
            elif isinstance(v, str) and v != "":
                cmd = f"--{k}={v}"
                self.extra_nuitka_options_list.append(cmd)
            elif type(v) is bool and v:
                cmd = f"--{k}"
                self.extra_nuitka_options_list.append(cmd)

        pyinstaller_config = glom.glom(data, "tool.pyside-cli.pyinstaller", default={})
        for k, v in pyinstaller_config.items():
            if isinstance(v, list) and v:
                cmd = f"--{k}={','.join(v)}"
                self.extra_pyinstaller_options_list.append(cmd)
            elif isinstance(v, str) and v != "":
                cmd = f"--{k}={v}"
                self.extra_pyinstaller_options_list.append(cmd)
            elif type(v) is bool and v:
                cmd = f"--{k}"
                self.extra_pyinstaller_options_list.append(cmd)

        self.lang_list = glom.glom(data, "tool.pyside-cli.i18n.languages", default=[])

        # Parse project.scripts
        self._parse_project_scripts(data)

    def _parse_project_scripts(self, data: dict):
        """
        Parse project.scripts from pyproject.toml and resolve package paths.
        
        For example:
        [project.scripts]
        pyside-cli = "cli.__main__:main"
        
        Will be parsed to:
        {"pyside-cli": "cli"}
        """
        scripts = glom.glom(data, "project.scripts", default={})

        for script_name, entry_point in scripts.items():
            resolved_path = _resolve_package_path(entry_point)
            self.scripts[script_name] = resolved_path
