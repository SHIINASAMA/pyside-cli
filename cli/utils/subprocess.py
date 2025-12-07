import os

import subprocess


def run_command(cmd, **kwargs):
    shell_mode = os.name == "nt"
    return subprocess.run(cmd, shell=shell_mode, **kwargs)
