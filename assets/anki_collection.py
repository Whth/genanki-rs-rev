import anki.collection
import tempfile

def setup(fname):
    import uuid
    colf_name = f"{fname}.anki2"
    return anki.collection.Collection(colf_name)