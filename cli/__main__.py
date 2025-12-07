import logging
import os
import sys

import colorama
from colorlog import ColoredFormatter

from cli.builder.build import gen_version_py, build
from cli.builder.qt import build_i18n_ts, build_i18n, build_ui, build_assets, gen_init_py
from cli.context.context import Context
from cli.create import create
from cli.git import get_last_tag
from cli.pytest import run_test


def main():
    ctx = Context()

    if sys.platform == "win32":
        colorama.just_fix_windows_console()

    logger = logging.getLogger()
    logger.setLevel(logging.INFO)
    handler = logging.StreamHandler()
    handler.setFormatter(ColoredFormatter(
        fmt='%(log_color)s%(asctime)s - %(levelname)s - %(message)s',
        log_colors={
            'DEBUG': 'cyan',
            'INFO': 'green',
            'WARNING': 'yellow',
            'ERROR': 'red',
            'CRITICAL': 'bold_red',
        }
    ))
    logger.handlers = []
    logger.addHandler(handler)

    if ctx.args.debug:
        logger.setLevel(logging.DEBUG)
        logging.info('Debug mode enabled.')

    if ctx.args.create:
        create()
        sys.exit(0)

    # check working directory
    # if 'app' is not in the current working directory, exit
    if not os.path.exists('app'):
        logging.error('Please run this script from the project root directory.')
        sys.exit(1)

    if ctx.args.test:
        code = run_test()
        sys.exit(code)
    else:
        ctx.glob_files()
        logging.debug("Source list: %s", [str(x) for x in ctx.source_list])
        logging.debug("UI list: %s", [str(x) for x in ctx.ui_list])
        logging.debug("Asset list: %s", [str(x) for x in ctx.asset_list])
        logging.debug("I18n list: %s", [str(x) for x in ctx.i18n_list])

    if ctx.args.no_cache:
        ctx.cache = {}

    if ctx.args.i18n:
        build_i18n_ts()
    if ctx.args.rc or ctx.args.all:
        build_ui()
        build_i18n()
        build_assets()
        gen_version_py(get_last_tag(ctx.toolchain))
        gen_init_py()
    ctx.save_cache()
    if ctx.args.build or ctx.args.all:
        build()


if __name__ == '__main__':
    main()
