"""Create writable extraction space while reusing the existing offline cache."""
from pathlib import Path


def reuse(source, destination):
    for index in source.iterdir() if source.exists() else []:
        target = destination / index.name
        target.mkdir(parents=True, exist_ok=True)
        for package in index.iterdir():
            link = target / package.name
            if not link.exists():
                link.symlink_to(package, target_is_directory=package.is_dir())


def prepare(source, target):
    source, target = Path(source).resolve(), Path(target).resolve()
    if source == target:
        return target
    (target / 'registry').mkdir(parents=True, exist_ok=True)
    index = target / 'registry/index'
    if not index.exists():
        index.symlink_to(source / 'registry/index', target_is_directory=True)
    reuse(source / 'registry/cache', target / 'registry/cache')
    reuse(source / 'registry/src', target / 'registry/src')
    return target
