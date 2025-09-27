import os
def check_media(col):
    # col.media.check seems to assume that the cwd is the media directory. So this helper function
    # chdirs to the media dir before running check and then goes back to the original cwd.
    orig_cwd = os.getcwd()
    os.chdir(col.media.dir())
    res = col.media.check()
    os.chdir(orig_cwd)
    return res.missing, res.report, res.unused