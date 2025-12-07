import argparse


def get_parser():
    """parse command line arguments"""
    # --help: show this help message
    parser = argparse.ArgumentParser(description='Test and build your app.')
    # --rc: only convert resources files to python files
    # --build: only build the app
    # --all: convert resources files and build the app
    # --test: run test code
    # --init: create new project
    mode_group = parser.add_mutually_exclusive_group(required=True)
    mode_group.add_argument('--all', action='store_true', help='Convert rc files and build the app')
    mode_group.add_argument('--build', action='store_true', help='Build the app')
    mode_group.add_argument('--create', type=str, metavar='NAME', help='Create your project with name')
    mode_group.add_argument('--i18n', action='store_true',
                            help='Generate translation files (.ts) for all languages')
    mode_group.add_argument('--rc', action='store_true', help='Convert rc files to python files')
    mode_group.add_argument('--test', action='store_true', help='Run test')
    # --onefile: create a single executable file
    # --onedir: create a directory with the executable and all dependencies
    package_format_group = parser.add_mutually_exclusive_group()
    package_format_group.add_argument('--onefile', action='store_true',
                                      help='(for build) Create a single executable file')
    package_format_group.add_argument('--onedir', action='store_true',
                                      help='(for build) Create a directory with the executable and all dependencies')

    parser.add_argument('--backend', metavar='BACKEND', type=str, help='(for build) Backend to use (Default: nuitka)',
                        choices=['nuitka', 'pyinstaller'], default='nuitka')

    parser.add_argument('--low-perf', action='store_true',
                        help='(for build) Use low performance mode, this may improve compatibility on some systems', )

    parser.add_argument('--no-cache', action='store_true', help='Ignore existing caches', required=False)

    parser.add_argument('--debug', action='store_true',
                        help='Enable debug mode, which will output more information during the build process')

    parser.add_argument('backend_args', nargs=argparse.REMAINDER,
                        help='Additional arguments for the build backend, e.g. -- --xxx=xxx')

    return parser
