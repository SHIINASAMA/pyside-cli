import logging
import os
import shutil
import stat
import subprocess
import sys
from pathlib import Path

import toml
from glom import assign, delete, glom

from cli.context.context import Context


def _remove_readonly(func, path, _):
    os.chmod(path, stat.S_IWRITE)
    func(path)


def _remove_git(path: Path):
    shutil.rmtree(path, onerror=_remove_readonly)


def create():
    ctx = Context()
    toolchain = ctx.toolchain
    name = ctx.args.create

    dst = name
    if name == '.':
        name = Path.cwd().name
        dst = '.'

    if toolchain.git_executable is None:
        logging.warning("Git executable not found, skipping.")
        sys.exit(-1)

    logging.info(f"Creating ...")

    shell_mode = os.name == "nt"
    rt = subprocess.run([
        toolchain.git_executable,
        'clone',
        'https://github.com/SHIINASAMA/pyside_template.git',
        dst],
        shell=shell_mode
    )
    if rt.returncode:
        logging.error('Failed to clone template.')
        return

    project_path = Path(dst)
    pyproject_file = project_path / 'pyproject.toml'

    with pyproject_file.open('r', encoding='utf-8') as f:
        data = toml.load(f)

    assign(data, 'project.name', name)
    value = glom(data, "project.scripts.pyside_template")
    delete(data, "project.scripts.pyside_template")
    assign(data, f'project.scripts.{name}', value)

    with pyproject_file.open('w', encoding='utf-8') as f:
        toml.dump(data, f)

    git_dir = project_path / '.git'
    _remove_git(git_dir)

    shell_mode = os.name == "nt"
    subprocess.run([
        'git',
        'init'],
        cwd=project_path,
        shell=shell_mode
    )

    logging.info(f"Project {name} created successfully.")
