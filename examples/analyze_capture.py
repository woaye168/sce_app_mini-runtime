# 分析 control_capture.jsonl：定位调试 host 控制会话，重组字节流并做 protobuf wire 解码
import json, sys

def varint(b, i):
    v = 0; s = 0
    while True:
        x = b[i]; i += 1
        v |= (x & 0x7F) << s
        if not (x & 0x80): break
        s += 7
    return v, i

def wire_dump(b, depth=0, maxdepth=3):
    """通用 protobuf wire 格式 dump（尽力而为）"""
    out = []
    i = 0
    pad = '  ' * depth
    while i < len(b):
        try:
            tag, i = varint(b, i)
        except Exception:
            out.append(f'{pad}<bad varint at {i}>'); break
        field = tag >> 3; wt = tag & 7
        if field == 0:
            out.append(f'{pad}<field0 at {i-1}>'); break
        if wt == 0:
            v, i = varint(b, i)
            out.append(f'{pad}f{field} varint = {v} ({hex(v)})')
        elif wt == 2:
            ln, i = varint(b, i)
            payload = b[i:i+ln]; i += ln
            # 判断：可打印字符串 or 嵌套消息
            printable = all(32 <= c < 127 or c in (9,10,13) for c in payload[:64]) and ln > 0
            if printable:
                out.append(f'{pad}f{field} len={ln} str = "{payload.decode("utf-8","replace")[:120]}"')
            elif depth < maxdepth and ln > 1:
                sub = wire_dump(payload, depth+1, maxdepth)
                if any('bad' not in s and 'field0' not in s for s in sub) and sub:
                    out.append(f'{pad}f{field} len={ln} msg = {{')
                    out.extend(sub)
                    out.append(f'{pad}}}')
                else:
                    out.append(f'{pad}f{field} len={ln} bytes = {payload[:32].hex()}{"..." if ln>32 else ""}')
            else:
                out.append(f'{pad}f{field} len={ln} bytes = {payload[:32].hex()}{"..." if ln>32 else ""}')
        elif wt == 5:
            out.append(f'{pad}f{field} i32 = {b[i:i+4].hex()}'); i += 4
        elif wt == 1:
            out.append(f'{pad}f{field} i64 = {b[i:i+8].hex()}'); i += 8
        else:
            out.append(f'{pad}<wt{wt} at {i} stop>'); break
    return out

def main(path):
    recs = [json.loads(l) for l in open(path, encoding='utf-8')]
    # socket -> addr 映射（connect 记录）
    sock2addr = {}
    for r in recs:
        if r['op'] == 'connect':
            sock2addr[r['sock']] = r['addr']
    # 找调试 host 的 socket（106.x）
    host_socks = {s for s, a in sock2addr.items() if a.startswith('106.')}
    print('connects:', {s: a for s, a in sock2addr.items()})
    # 给 WSASend 记录补 addr（recv 记录有 addr，send/WSASend 靠 sock）
    # 控制连接 = TCP 到 106.x。recv 带 addr 的直接可用；WSASend addr='?' 的需要 sock 匹配
    for r in recs:
        if r.get('addr') == '?' and r.get('sock') in sock2addr:
            r['addr'] = sock2addr[r['sock']]
    # 按 socket 分组时间线
    timeline = [r for r in recs if r.get('addr','').startswith('106.') or r.get('sock') in host_socks]
    print(f'\n=== control session records: {len(timeline)} ===')
    for idx, r in enumerate(timeline):
        op = r['op']; ln = r.get('len', 0)
        print(f'\n--- [{idx}] {op} sock={r.get("sock")} addr={r.get("addr")} len={ln}')
        data = r.get('data')
        if not data or not data.startswith('<') is False:
            pass
        if data:
            b = bytes.fromhex(data)
            print('hex:', data[:200] + ('...' if len(data) > 200 else ''))
            if op in ('send', 'WSASend', 'recv', 'WSARecv') and ln > 0:
                for line in wire_dump(b):
                    print(line)

if __name__ == '__main__':
    main(sys.argv[1])
