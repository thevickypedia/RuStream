import json
import os
import pathlib
import re
from typing import Dict, List, Union


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
    return json.dumps({"files": sorted(files, key=lambda x: natural_sort_key(x['name']))})


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
    return json.dumps(dict(files=sorted(structure['files'], key=lambda x: natural_sort_key(x['name'])),
                           directories=sorted(structure['directories'], key=lambda x: natural_sort_key(x['name']))))


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


def srt_to_vtt(filename: str) -> str:
    if not filename.endswith('.srt'):
        return json.dumps(False)
    filename = pathlib.PosixPath(filename)
    output_file = filename.with_suffix('.vtt')
    with open(filename, 'r', encoding='utf-8') as rf:
        srt_content = rf.read()
    srt_content = srt_content.replace(',', '.')
    srt_content = srt_content.replace(' --> ', '-->')
    vtt_content = 'WEBVTT\n\n'
    subtitle_blocks = srt_content.strip().split('\n\n')
    for block in subtitle_blocks:
        lines = block.split('\n')
        timecode = lines[1]
        text = '\n'.join(lines[2:])
        vtt_content += f"{timecode}\n{text}\n\n"
    with open(output_file, 'w', encoding='utf-8') as wf:
        wf.write(vtt_content)
        wf.flush()
    if output_file.exists():
        return json.dumps(True)
    return json.dumps(False)
