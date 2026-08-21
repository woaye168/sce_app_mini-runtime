import base64, json, sys
ui = json.load(open(sys.argv[1], encoding='utf-8'))
tok = ui['token']
kid = tok.split('$', 1)[1]
pad = '=' * (-len(kid) % 4)
raw = base64.urlsafe_b64decode(kid + pad)
print('kid bytes len', len(raw))
print(raw[:80].hex())
print(repr(raw[:200]))
