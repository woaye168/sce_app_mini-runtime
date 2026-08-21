# 从 control_capture.jsonl 提取调试 host 控制会话的全部帧（含大 send 里的连续帧）
# 帧格式（已实证）: u32 LE total_len(含自身4字节) + u8 0x00 + proto{ f1(wt2): { f1 varint msg_type; f2(wt2) body } }
import json, sys

MSG_NAMES = {
    0xF000: 'EditorLogin', 0xF001: 'EditorLoginResult',
    0xF004: 'SendWriteFile', 0xF010: 'SendWriteFileAck',
    0xF00C: 'NotifyEditorLog', 0xF011: 'EditorPing', 0xF017: 'EditorPingRes',
    0xF018: '?0xF018', 0xF01A: '?0xF01A',
}

def varint(b, i):
    v = 0; s = 0
    while True:
        x = b[i]; i += 1
        v |= (x & 0x7F) << s
        if not (x & 0x80): break
        s += 7
    return v, i

def field(b, i):
    tag, i = varint(b, i)
    fn, wt = tag >> 3, tag & 7
    if wt == 0:
        v, i = varint(b, i); return fn, wt, v, i
    if wt == 2:
        ln, i = varint(b, i); v = b[i:i+ln]; return fn, wt, v, i + ln
    if wt == 5:
        return fn, wt, b[i:i+4], i + 4
    if wt == 1:
        return fn, wt, b[i:i+8], i + 8
    raise ValueError(f'wt{wt}')

def dump_body(b, indent='    '):
    i = 0; out = []
    while i < len(b):
        try:
            fn, wt, v, i = field(b, i)
        except Exception as e:
            out.append(f'{indent}<parse stop at {i}: {e}>'); break
        if wt == 0:
            out.append(f'{indent}f{fn} varint = {v} ({hex(v)})')
        elif wt == 2:
            printable = len(v) > 0 and all(32 <= c < 127 or c in (9, 10, 13) for c in v[:96])
            if printable:
                out.append(f'{indent}f{fn} len={len(v)} str = "{v.decode("utf-8","replace")[:160]}"')
            else:
                out.append(f'{indent}f{fn} len={len(v)} bytes = {v[:48].hex()}{"..." if len(v)>48 else ""}')
        else:
            out.append(f'{indent}f{fn} wt{wt} = {v.hex() if isinstance(v,bytes) else v}')
    return out

def frames_of(data):
    """切分出全部帧，返回 [(msg_type, body_bytes, frame_len)]"""
    out = []
    i = 0
    while i + 5 <= len(data):
        total = int.from_bytes(data[i:i+4], 'little')
        if total < 6 or i + total > len(data):
            out.append(('BADFRAME', data[i:i+64], total)); break
        frame = data[i:i+total]
        i += total
        # frame: total(4) + 00 + 0a <len> { 08 <type> 12 <len> body }
        try:
            j = 4
            flag = frame[j]; j += 1
            fn, wt, env, j = field(frame, j)   # f1 wt2 envelope
            k = 0
            fn1, wt1, mtype, k = field(env, k)  # f1 varint type
            fn2, wt2, body, k = field(env, k)   # f2 wt2 body
            out.append((mtype, body, total))
        except Exception as e:
            out.append(('PARSE_ERR', frame[:64], str(e)))
    return out

def main(path):
    recs = [json.loads(l) for l in open(path, encoding='utf-8')]
    sock2addr = {r['sock']: r['addr'] for r in recs if r['op'] == 'connect'}
    host_socks = [s for s, a in sock2addr.items() if a.startswith('106.')]
    print('host socks:', {s: sock2addr[s] for s in host_socks})
    hs = set(host_socks)
    for r in recs:
        if r.get('sock') in hs and r['op'] in ('send', 'recv', 'WSASend', 'WSARecv'):
            data = bytes.fromhex(r.get('data') or '')
            for mtype, body, total in frames_of(data):
                name = MSG_NAMES.get(mtype, f'type_{hex(mtype) if isinstance(mtype,int) else mtype}')
                print(f'\n== {r["op"]} {name} (frame {total}B, body {len(body)}B)')
                for line in dump_body(body):
                    print(line)

if __name__ == '__main__':
    main(sys.argv[1])
