import json
import logging
import os
from pathlib import Path

from singleton_decorator import singleton

from .args import get_parser
from .pyproject import PyProjectConfig
from .toolchain import Toolchain


def _load_cache():
    """load cache from .cache/assets.json"""
    cache = {}
    if not os.path.exists('.cache'):
        os.makedirs('.cache')
    if os.path.exists('.cache/assets.json'):
        logging.info('Cache found.')
        with open('.cache/assets.json', 'r') as f:
            cache = json.load(f)
    if not cache:
        logging.info('No cache found.')
    return cache


@singleton
class Context:
    source_list = []
    ui_list = []
    asset_list = []
    i18n_list = []

    target_name = "App"
    target_dir = "app"

    def __init__(self):
        self.args, self.args.backend_args = get_parser().parse_known_args()
        if self.args.backend_args and self.args.backend_args[0] == "--":
            self.args.backend_args = self.args.backend_args[1:]

        self.config = PyProjectConfig()
        self.toolchain = Toolchain()
        self.cache = _load_cache()

    def detect_target(self):
        if self.args.target:
            self.target_name = self.args.target
            if self.target_name not in self.config.scripts:
                raise ValueError(f"Target '{self.target_name}' not found in pyproject.toml scripts.")
            self.target_dir = self.config.scripts[self.target_name]
            if not os.path.exists(self.target_dir):
                raise ValueError(f"Target directory '{self.target_dir}' does not exist.")

    def glob_files(self):
        root = Path(self.target_dir)
        assets_dir = root / "assets"
        i18n_dir = root / "i18n"
        exclude_dirs = [
            root / "resources",
            root / "test"
        ]

        for path in root.rglob("*"):
            if any(ex in path.parents for ex in exclude_dirs):
                continue

            if assets_dir in path.parents and os.path.isfile(path):
                self.asset_list.append(path)
                continue

            if i18n_dir in path.parents and os.path.isfile(path):
                self.i18n_list.append(path)
                continue

            if path.suffix == ".py":
                self.source_list.append(path)
            elif path.suffix == ".ui":
                self.ui_list.append(path)

    def save_cache(self):
        # save cache
        with open('.cache/assets.json', 'w') as f:
            json.dump(self.cache, f, indent=4)
        logging.info('Cache saved.')
