import glob
import os

DIR = '/var/lib/merkava/data'


def clean():
    files = glob.glob(f'{DIR}/*.pkt')
    print(f'cleaning {len(files)} files')
    for file_name in files:
        os.remove(file_name)

    return True


def isalive():
    return True
