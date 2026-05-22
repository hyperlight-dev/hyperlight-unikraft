"""Minimal pydoc stub — provides getdoc() for pyarrow's vendored docscrape."""

def getdoc(obj):
    try:
        doc = obj.__doc__
    except AttributeError:
        return ''
    if not doc:
        return ''
    import re
    return re.sub('^ *\n', '', doc.rstrip())
