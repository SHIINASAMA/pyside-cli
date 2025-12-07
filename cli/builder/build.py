import logging
import os
import shutil
import sys
import time
from pathlib import Path

from cli.builder.backend import NuitkaBackend, PyInstallerBackend
from cli.context.context import Context
from cli.utils.subprocess import run_command


def gen_version_py(version):
    ctx = Context()
    with open(f'{ctx.target_dir}/resources/version.py', 'w', encoding='utf-8') as f:
        f.write(f'__version__ = "{version}"\n')


def _gen_filelist(root_dir: str):
    filelist_name = f'{root_dir}/filelist.txt'
    paths = []
    for current_path, dirs, files in os.walk(root_dir, topdown=False):
        for file in files:
            relative_path = os.path.relpath(os.path.join(current_path, file), root_dir)
            logging.debug(relative_path)
            paths.append(relative_path)
        relative_path = os.path.relpath(os.path.join(current_path, ""), root_dir)
        if relative_path != ".":
            logging.debug(relative_path)
            paths.append(relative_path)

    with open(filelist_name, "w", encoding="utf-8") as f:
        f.write("\n".join(paths))
        f.write("\n")


def build():
    """call backend to build the app"""
    ctx = Context()
    build_path_str = f'build/{ctx.target_name}'  # build/App
    if sys.platform != 'win32':
        path = Path(build_path_str)
        if path.exists() and path.is_dir():
            shutil.rmtree(path)
        elif path.exists() and path.is_file():
            path.unlink()
    start = time.perf_counter()
    logging.info('Building the app...')

    if ctx.args.backend == 'nuitka':
        backend = NuitkaBackend()
    else:
        backend = PyInstallerBackend()
    logging.debug(' '.join(backend.get_cmd()))
    try:
        result = run_command(backend.get_cmd())
        end = time.perf_counter()
        if result.returncode != 0:
            logging.error(f'Failed to build app in {end - start:.3f}s.')
            sys.exit(1)
        logging.info(f'Build complete in {end - start:.3f}s.')
        backend.post_build()
        if not ctx.args.onefile:
            logging.info("Generate the filelist.")
            _gen_filelist(build_path_str)
            logging.info("Filelist has been generated.")
    except Exception as e:
        end = time.perf_counter()
        logging.error(f'Exception during build: {e}, time: {end - start:.3f}s.')
        sys.exit(1)
