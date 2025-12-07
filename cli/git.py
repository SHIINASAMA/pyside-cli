import logging
import subprocess

from cli.context.context import Context


def get_last_tag(default="0.0.0.0") -> str:
    """Get the last git tag as version, or return default if not found."""
    ctx = Context()
    toolchain = ctx.toolchain

    if toolchain.git_executable is None:
        logging.warning("Git executable not found, skipping get tag.")
        return default
    try:
        tag = subprocess.check_output(
            ["git", "describe", "--tags", "--abbrev=0", "--first-parent"],
            stderr=subprocess.DEVNULL,
            text=True,
            shell=True
        ).strip()
    except subprocess.CalledProcessError:
        return default

    return tag if tag else default
