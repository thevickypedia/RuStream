import os
import json
import pathlib
import re

from datetime import datetime
from typing import Dict, List, Tuple, Union


def natural_sort_key(filename: str) -> List[Union[int, str]]:
    parts = re.split(r'(\d+)', filename)
    return [int(part) if part.isdigit() else part.lower() for part in parts]


def get_dir_stream_content(parent: str, subdir: str, file_formats: List[str]) -> List[Dict[str, str]]:
    files = []
    for file_ in os.listdir(parent):
        if file_.startswith('_') or file_.startswith('.'):
            continue
        if pathlib.PurePath(file_).suffix in file_formats:
            files.append({"name": file_, "path": os.path.join(subdir, file_)})
    data = sorted(files, key=lambda x: natural_sort_key(x['name']))
    filename = datetime.now().strftime(os.path.join(os.getcwd(), 'temp_dir_%d-%m-%Y_%H:%M:%S.json'))
    with open(filename, "w") as file:
        json.dump({"files": data}, file)
        file.flush()
    return filename



def get_all_stream_content(video_source: str, file_formats: List[str]) -> Dict[str, List[Dict[str, str]]]:
    structure = {'files': [], 'directories': []}
    for __path, __directory, __file in os.walk(video_source):
        if __path.endswith('__'):
            continue
        for file_ in __file:
            if file_.startswith('_') or file_.startswith('.'):
                continue
            if pathlib.PurePath(file_).suffix in file_formats:
                if path := __path.replace(video_source, "").lstrip(os.path.sep):
                    entry = {"name": path, "path": os.path.join("stream", path)}
                    if entry in structure['directories']:
                        continue
                    structure['directories'].append(entry)
                else:
                    structure['files'].append({"name": file_, "path": os.path.join("stream", file_)})
    data = dict(files=sorted(structure['files'], key=lambda x: natural_sort_key(x['name'])),
                directories=sorted(structure['directories'], key=lambda x: natural_sort_key(x['name'])))
    filename = datetime.now().strftime(os.path.join(os.getcwd(), 'temp_all_%d-%m-%Y_%H:%M:%S.json'))
    with open(filename, "w") as file:
        json.dump(data, file)
        file.flush()
    return filename


def get_iter(filepath: str, file_formats: List[str]) -> Union[List[str], List[None]]:
    filepath = pathlib.PosixPath(filepath)
    # Extract only the file formats that are supported
    dir_content = sorted(
        (file for file in os.listdir(filepath.parent) if pathlib.PosixPath(file).suffix in file_formats),
        key=lambda x: natural_sort_key(x)
    )
    idx = dir_content.index(filepath.name)
    if idx > 0:
        try:
            previous_ = dir_content[idx - 1]
            if previous_ == filepath.name:
                previous_ = None
        except IndexError:
            previous_ = None
    else:
        previous_ = None
    try:
        next_ = dir_content[idx + 1]
    except IndexError:
        next_ = None
    return json.dumps({"previous": previous_, "next": next_})
