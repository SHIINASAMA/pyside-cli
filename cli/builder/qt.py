import logging
import os
import subprocess
import sys
from concurrent.futures import as_completed
from concurrent.futures.process import ProcessPoolExecutor
from pathlib import Path

from cli.context.context import Context


def _compile_ui(uic, input_file: Path, output_file: Path):
    """Compile .ui files to .py files
    This function will be called in build_ui via `ProcessPoolExecutor`"""
    cmd = [
        uic,
        str(input_file),
        "-o",
        str(output_file)]
    logging.debug(" ".join(cmd))
    shell_mode = os.name == "nt"
    try:
        result = subprocess.run(
            cmd,
            shell=shell_mode
        )
        success = result.returncode == 0
        if not success:
            logging.debug(result.stderr)
        return success, input_file, output_file
    except subprocess.CalledProcessError:
        return False, input_file, output_file


def build_ui():
    """Compile *.ui files into Python files using pyside6-uic, preserving directory structure."""
    ctx = Context()
    toolchain = ctx.toolchain
    ui_list = ctx.ui_list
    cache = ctx.cache
    low_perf_mode = ctx.args.low_perf

    if toolchain.uic_executable is None:
        logging.warning("PySide6 uic not found, exiting.")
        sys.exit(-1)

    ui_dir = Path("app/ui")
    res_dir = Path("app/resources")

    if not ui_list:
        logging.info("No ui files found, skipping ui conversion.")
        return

    res_dir.mkdir(parents=True, exist_ok=True)
    logging.info("Converting ui files to Python files...")
    ui_cache = cache.get("ui", {})

    args_pair = []
    for input_file in ui_list:
        try:
            rel_path = input_file.parent.relative_to(ui_dir)
        except ValueError:
            # input_file is not under app/ui, skip it
            continue

        output_dir = res_dir / rel_path
        output_dir.mkdir(parents=True, exist_ok=True)

        output_file = output_dir / (input_file.stem + "_ui.py")

        # Check cache to avoid unnecessary recompilation
        mtime = input_file.stat().st_mtime
        if str(input_file) in ui_cache and ui_cache[str(input_file)] == mtime:
            logging.info(f"{input_file} is up to date.")
            continue

        ui_cache[str(input_file)] = mtime

        if low_perf_mode:
            _compile_ui(toolchain.uic_executable, input_file, output_file)
        else:
            # Collect files that need reconvert
            args_pair.append((input_file, output_file))

    has_error = False
    if args_pair:
        with ProcessPoolExecutor() as executor:
            futures = [executor.submit(_compile_ui, toolchain.uic_executable, i, o) for i, o in args_pair]
            for f in as_completed(futures):
                ok, i, o = f.result()
                if ok:
                    logging.info(f"Converted {i} to {o}.")
                else:
                    logging.error(f"Failed to convert {i}.")
                    has_error = True
                    break
            if has_error:
                for future in futures:
                    future.cancel()
                for p in executor._processes.values():
                    p.terminate()

    cache["ui"] = ui_cache
    if has_error:
        sys.exit(-1)


def build_assets():
    """Generate assets.qrc from files in app/assets and compile it with pyside6-rcc."""
    ctx = Context()
    toolchain = ctx.toolchain
    asset_list = ctx.asset_list
    cache = ctx.cache
    no_cache = ctx.args.no_cache

    if toolchain.rcc_executable is None:
        logging.warning('PySide6 rcc not found, exiting.')
        sys.exit(-1)

    assets_dir = Path('app/assets')
    res_dir = Path('app/resources')
    qrc_file = res_dir / 'assets.qrc'
    py_res_file = res_dir / 'resource.py'

    # Skip if assets directory does not exist
    if not os.path.exists(assets_dir):
        logging.info('No assets folder found, skipping assets conversion.')
        return

    if not os.path.exists(res_dir):
        os.makedirs(res_dir)

    logging.info('Converting assets to Python resources...')

    assets_cache = cache.get('assets', {})
    need_rebuild = False
    for asset in asset_list:
        mtime = asset.stat().st_mtime
        asset_key = str(asset)
        if asset_key in assets_cache and assets_cache[asset_key] == mtime:
            logging.info(f'{asset} is up to date.')
            continue
        assets_cache[asset_key] = mtime
        logging.info(f'{asset} is outdated.')
        need_rebuild = True

    # Force rebuild if cache is disabled
    if no_cache:
        need_rebuild = True

    if need_rebuild:
        # Generate assets.qrc dynamically
        with open(qrc_file, 'w', encoding='utf-8') as f:
            f.write('<!DOCTYPE RCC>\n')
            f.write('<RCC version="1.0">\n')
            f.write('  <qresource>\n')
            for asset in asset_list:
                posix_path = asset.as_posix()
                # remove the leading "app/assets/" from the path
                alias = posix_path[len('app/assets/'):]
                # rel_path is the path relative to app/resources
                rel_path = os.path.relpath(asset, res_dir)
                f.write(f'    <file alias="{alias}">{rel_path}</file>\n')
            f.write('  </qresource>\n')
            f.write('</RCC>\n')
        logging.info(f'Generated {qrc_file}.')

        # Compile qrc file to Python resource
        cmd = [
            toolchain.rcc_executable,
            str(qrc_file),
            "-o",
            str(py_res_file)
        ]
        logging.debug(" ".join(cmd))
        shell_mode = os.name == "nt"
        result = subprocess.run(
            cmd,
            shell=shell_mode
        )
        if 0 != result.returncode:
            logging.error('Failed to convert assets.qrc.')
            exit(1)
        logging.info(f'Converted {qrc_file} to {py_res_file}.')
    else:
        logging.info('Assets are up to date.')

    cache['assets'] = assets_cache


def build_i18n_ts():
    """
    Generate translation (.ts) files for all languages in lang_list
    by scanning self.ui_list and self.source_list using pyside6-lupdate.
    """
    ctx = Context()
    toolchain = ctx.toolchain
    cache = ctx.cache
    lang_list = ctx.config.lang_list
    files_to_scan = [str(f) for f in ctx.ui_list + ctx.source_list]

    if toolchain.lupdate_executable is None:
        logging.warning("PySide6 lupdate not found, skipping i18n generation.")
        return

    i18n_dir = Path("app/i18n")
    i18n_dir.mkdir(parents=True, exist_ok=True)

    logging.info("Generating translation (.ts) files for languages: %s", ', '.join(lang_list))

    i18n_cache = cache.get("i18n", {})

    for lang in lang_list:
        ts_file = i18n_dir / f"{lang}.ts"
        logging.info("Generating %s ...", ts_file)

        # files_str = " ".join(f'"{f}"' for f in files_to_scan)
        # cmd = f'pyside6-lupdate -silent -locations absolute -extensions ui {files_str} -ts "{ts_file}"'

        cmd = [
            toolchain.lupdate_executable,
            "-silent",
            "-locations", "absolute",
            "-extensions", "ui",
            *files_to_scan,
            "-ts", str(ts_file)
        ]
        logging.debug(" ".join(cmd))
        shell_mode = os.name == "nt"
        result = subprocess.run(cmd, shell=shell_mode, env=os.environ.copy())
        if 0 != result.returncode:
            logging.error("Failed to generate translation file: %s.", ts_file)
            exit(1)

        i18n_cache[lang] = ts_file.stat().st_mtime
        logging.info("Generated translation file: %s.", ts_file)

    cache["i18n"] = i18n_cache


def _compile_qm(lrelease, input_file: Path, output_file: Path):
    """Compile .ts files to .qm files
    This function will be called in build_i18n via `ProcessPoolExecutor`"""
    logging.info(f"Compiling {input_file} to {output_file}.")
    cmd = [
        lrelease,
        str(input_file),
        "-qm",
        str(output_file)]
    logging.debug(" ".join(cmd))
    shell_mode = os.name == "nt"
    try:
        result = subprocess.run(
            cmd,
            shell=shell_mode
        )
        success = result.returncode == 0
        if not success:
            logging.debug(result.stderr)
        return success, input_file, output_file
    except subprocess.CalledProcessError:
        return False, input_file, output_file


def build_i18n():
    """
    Compile .ts translation files into .qm files under app/assets/i18n/.
    Only regenerate .qm if the corresponding .ts file has changed.
    """
    ctx = Context()
    toolchain = ctx.toolchain
    i18n_list = ctx.i18n_list
    cache = ctx.cache
    low_perf_mode = ctx.args.low_perf

    if toolchain.lrelease_executable is None:
        logging.warning("PySide6 lrelease not found, skipping i18n compilation.")
        return

    qm_root = Path("app/assets/i18n")
    qm_root.mkdir(parents=True, exist_ok=True)

    logging.info("Compiling translation files...")

    # Get cache for i18n
    i18n_cache = cache.get("i18n", {})

    args_pair = []
    for ts_file in i18n_list:
        try:
            ts_file = Path(ts_file)
        except Exception:
            continue

        qm_file = qm_root / (ts_file.stem + ".qm")

        # Check modification time cache
        ts_mtime = ts_file.stat().st_mtime
        if str(ts_file) in i18n_cache and i18n_cache[str(ts_file)] == ts_mtime:
            logging.info("%s is up to date.", ts_file)
            continue

        # Update cache
        i18n_cache[str(ts_file)] = ts_mtime
        if low_perf_mode:
            _compile_qm(toolchain.lrelease_executable, ts_file, qm_file)
        else:
            args_pair.append((ts_file, qm_file))

    has_error = False
    if args_pair:
        with ProcessPoolExecutor() as executor:
            futures = [executor.submit(_compile_qm, toolchain.lrelease_executable, i, o) for i, o in args_pair]
            for f in as_completed(futures):
                ok, i, _ = f.result()
                if ok:
                    logging.info(f"Compiled {i}.")
                else:
                    logging.error(f"Failed to compile translation file: {i}.")
                    has_error = True
                    break
            if has_error:
                for future in futures:
                    future.cancel()
                for p in executor._processes.values():
                    p.terminate()

    cache["i18n"] = i18n_cache
    if has_error:
        sys.exit(-1)


def gen_init_py():
    """Create __init__.py in every subdirectory if not exists"""
    root = Path("app/resources")
    init_file = root / "__init__.py"
    if not init_file.exists():
        init_file.touch()
    for path in root.rglob("*"):
        if path.is_dir():
            init_file = path / "__init__.py"
            if not init_file.exists():
                init_file.touch()
