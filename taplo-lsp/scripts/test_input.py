import json
import sys
from typing import Optional


def encode(val):
    msg = json.dumps(val)
    l = len(msg.encode('utf-8'))
    return f'Content-Length: {l}\r\n\r\n{msg}\r\n'


def set_optional(name: str, d, args):
    v = args.get(name)
    if v is not None:
        d[name] = v


def message(method: str, **kwargs):
    m = {
        'jsonrpc': '2.0',
        'method': method,
    }

    set_optional('id', m, kwargs)
    set_optional('params', m, kwargs)

    return m

# TODO: proper tests


test_messages = [
    message("initialize", params={"capabilities": {}}, id=1),
    message("shutdown", id=2),
    message("exit")
]

for message in test_messages:
    sys.stdout.write(encode(message))
