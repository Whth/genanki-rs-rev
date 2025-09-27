import os
import time
import shutil
def cleanup(fname, col):
    col.close()
    path = col.path
    media = path.split(".anki2")[0] + '.media'
    os.remove(path)
    shutil.rmtree(media)