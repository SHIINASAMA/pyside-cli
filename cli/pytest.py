import logging

from cli.context.context import Context
from cli.utils.subprocess import run_command


def run_test():
    ctx = Context()
    toolchain = ctx.toolchain
    args = ctx.args

    if toolchain.pytest_executable is None:
        logging.warning("Pytest executable not found, skipping test.")
        return -1
    cmd = ['pytest'] + args.backend_args
    logging.debug(' '.join(cmd))
    result = run_command(cmd)
    return result.returncode
