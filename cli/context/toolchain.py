import logging
import os
import shutil
import site


def _add_path():
    appended_path = ""
    items = site.getsitepackages()

    package_names = [
        "PySide6"
    ]

    for item in items:
        for package_name in package_names:
            full_path = os.path.join(item, package_name)
            if os.path.exists(full_path):
                appended_path += os.path.pathsep + full_path

    os.environ["PATH"] += appended_path


class Toolchain:
    git_executable = None
    uic_executable = None
    rcc_executable = None
    lupdate_executable = None
    lrelease_executable = None
    nuitka_executable = None
    pyinstaller_executable = None
    pytest_executable = None

    def __init__(self):
        _add_path()
        self.git_executable = shutil.which("git")
        self.uic_executable = shutil.which("pyside6-uic")
        self.rcc_executable = shutil.which("pyside6-rcc")
        self.lupdate_executable = shutil.which("lupdate")
        self.lrelease_executable = shutil.which("lrelease")
        self.nuitka_executable = shutil.which("nuitka")
        self.pyinstaller_executable = shutil.which("pyinstaller")
        self.pytest_executable = shutil.which("pytest")

    def print_toolchain(self):
        logging.info("Found toolchain:")

        logging.info(f"GIT: {self.git_executable is not None}")
        logging.info(f"UIC: {self.uic_executable is not None}")
        logging.info(f"RCC: {self.rcc_executable is not None}")
        logging.info(f"LUPDATE: {self.lupdate_executable is not None}")
        logging.info(f"LRELEASE: {self.lrelease_executable is not None}")
        logging.info(f"NUITKA: {self.nuitka_executable is not None}")
        logging.info(f"PYINSTALLER: {self.pyinstaller_executable is not None}")
        logging.info(f"PYTEST: {self.pytest_executable is not None}")

        logging.debug(f"Path to GIT: {self.git_executable}")
        logging.debug(f"Path to UIC: {self.uic_executable}")
        logging.debug(f"Path to RCC: {self.rcc_executable}")
        logging.debug(f"Path to LUPDATE: {self.lupdate_executable}")
        logging.debug(f"Path to LRELEASE: {self.lrelease_executable}")
        logging.debug(f"Path to NUITKA: {self.nuitka_executable}")
        logging.debug(f"Path to PYINSTALLER: {self.pyinstaller_executable}")
        logging.debug(f"Path to PYTEST: {self.pytest_executable}")
