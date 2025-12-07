import logging
import os
import shutil
from abc import ABC, abstractmethod
from typing import List

from cli.context.context import Context


class Backend(ABC):

    @abstractmethod
    def pre_build(self) -> bool:
        pass

    @abstractmethod
    def post_build(self) -> bool:
        pass

    @abstractmethod
    def get_cmd(self) -> List[str]:
        pass


class NuitkaBackend(Backend):
    def __init__(self):
        ctx = Context()
        self.cmd = [
            ctx.toolchain.nuitka_executable,
            '--output-dir=build',
            '--output-filename=App',
            ctx.target_dir,
            f'--jobs={os.cpu_count()}',
            '--onefile' if ctx.args.onefile else '--standalone'
        ]
        if ctx.args.debug:
            lt = [
                # '--show-scons',
                '--verbose',
            ]
            self.cmd.extend(lt)
        self.cmd.extend(ctx.config.extra_nuitka_options_list)
        self.cmd.extend(ctx.args.backend_args)

    def get_cmd(self) -> List[str]:
        return self.cmd

    def pre_build(self):
        ctx = Context()
        if ctx.toolchain.nuitka_executable is None:
            logging.warning('Nuitka executable not found, please install Nuitka first.')
            return False
        return True

    def post_build(self):
        ctx = Context()
        build_path_str = f'build/{ctx.target_name}'
        if not ctx.args.onefile:
            if os.path.exists(build_path_str):
                shutil.rmtree(build_path_str)
            shutil.move(f'build/{ctx.target_dir}.dist', build_path_str)
        pass


class PyInstallerBackend(Backend):
    def __init__(self):
        ctx = Context()
        workpath = 'build/' + ('pyinstaller_onefile_build' if ctx.args.onefile else 'pyinstaller_onedir_build')
        self.cmd = [
            ctx.toolchain.pyinstaller_executable,
            '--onefile' if ctx.args.onefile else '--onedir',
            '--distpath', 'build',
            '--workpath', workpath,
            '--noconfirm',
            '--log-level', 'DEBUG' if ctx.args.debug else 'WARN',
            '--name', ctx.target_name, f'{ctx.target_dir}/__main__.py',
        ]
        self.cmd.extend(ctx.config.extra_pyinstaller_options_list)
        self.cmd.extend(ctx.args.backend_args)

    def get_cmd(self) -> List[str]:
        return self.cmd

    def pre_build(self):
        ctx = Context()
        if ctx.toolchain.pyinstaller_executable is None:
            logging.warning('PyInstaller executable not found, please install PyInstaller first.')
            return False
        return True

    def post_build(self):
        ctx = Context()
        if not ctx.args.onefile:
            shutil.rmtree(f'{ctx.target_name}.spec', ignore_errors=True)
