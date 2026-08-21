# 从抓包提取大文件分块序列的精确字段布局（首块/中间块/尾块/FileEnd）
import json, sys

def varint(b, i):
    v = 0; s = 0
    while True:
        x = b[i]; i += 1
        v |= (x & 0x7F) << s
        if not (x & 0x80): break
        s += 7
    return v, i

def parse_fields(b):
    i = 0; out = []
    while i < len(b):
        tag, i = varint(b, i)
        fn, wt = tag >> 3, tag & 7
        if wt == 0:
            v, i = varint(b, i); out.append((fn, 'varint', v))
        elif wt == 2:
            ln, i = varint(b, i); out.append((fn, 'bytes', b[i:i+ln])); i += ln
        elif wt == 5:
            out.append((fn, 'i32', b[i:i+4])); i += 4
        elif wt == 1:
            out.append((fn, 'i64', b[i:i+8])); i += 8
        else:
            out.append((fn, f'wt{wt}', None)); break
    return out

def frames_of(data):
    out = []; i = 0
    while i + 5 <= len(data):
        total = int.from_bytes(data[i:i+4], 'little')
        if total < 6 or i + total > len(data): break
        frame = data[i:i+total]; i += total
        j = 5  # skip total(4)+flag(1)
        tag, j = varint(frame, j)
        elen, j = varint(frame, j)
        env = frame[j:j+elen]
        k = 0
        t1, k = varint(env, k); mtype, k = varint(env, k)
        t2, k = varint(env, k); blen, k = varint(env, k)
        body = env[k:k+blen]
        out.append((mtype, body))
    return out

def main(path):
    recs = [json.loads(l) for l in open(path, encoding='utf-8')]
    sock2addr = {r['sock']: r['addr'] for r in recs if r['op'] == 'connect'}
    hs = {s for s, a in sock2addr.items() if a.startswith('106.')}
    seq = []
    for r in recs:
        if r.get('sock') in hs and r['op'] == 'send':
            for mtype, body in frames_of(bytes.fromhex(r.get('data') or '')):
                seq.append((mtype, body))
    # 按文件分组：跟踪每个 path 的消息序列
    print(f'total frames: {len(seq)}')
    # 找一个多块文件的完整序列
    cur_path = None
    count = 0
    for mtype, body in seq:
        if mtype in (0xF004, 0xF008, 0xF00A):
            fs = parse_fields(body)
            path = next((v.decode('utf-8', 'replace') for f, t, v in fs if f == 1 and t == 'bytes'), '?')
            if path != cur_path:
                cur_path = path
                count = 0
                print(f'\n### {path}')
            count += 1
            desc = []
            for f, t, v in fs:
                if t == 'bytes' and len(v) > 200:
                    desc.append(f'f{f}=bytes[{len(v)}] md5?={v[:8].hex()}...')
                elif t == 'bytes':
                    desc.append(f'f{f}="{v.decode("utf-8","replace")}"')
                else:
                    desc.append(f'f{f}={v}')
            print(f'  [{count}] type={hex(mtype)}: ' + ' '.join(desc))
            if count > 30:
                print('  ...(截断)')
                cur_path = None  # 强制下一个文件

if __name__ == '__main__':
    main(sys.argv[1])
