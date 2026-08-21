# 从 protobuf C++ 二进制（sceengine.dll）提取内嵌的 FileDescriptorProto
# 原理：protoc 生成的代码内嵌序列化 FileDescriptorProto，blob 以 0x0A <varint len> "<name>.proto" 开头
import sys, re
from google.protobuf import descriptor_pb2

def main(path, out_dir, name_filter=None):
    data = open(path, 'rb').read()
    print(f'file size: {len(data)}')
    hits = []
    pos = 0
    while True:
        i = data.find(b'.proto', pos)
        if i < 0:
            break
        hits.append(i)
        pos = i + 1
    print(f'.proto hits: {len(hits)}')
    seen = set()
    for h in hits:
        # 向前找文件名字符串起点（可打印字符）
        s = h
        while s > 0 and 32 <= data[s-1] < 127:
            s -= 1
        fname = data[s:h+6]
        if not fname.endswith(b'.proto'):
            continue
        # blob 起点 = 0x0A <varint len> 正好编码 fname 长度
        start = None
        flen = len(fname)
        for back in range(1, 6):
            p = s - back
            if p < 0 or data[p] != 0x0A:
                continue
            # 解析 varint 长度
            val = 0; shift = 0; q = p + 1
            while True:
                b = data[q]; q += 1
                val |= (b & 0x7F) << shift
                if not (b & 0x80):
                    break
                shift += 7
            if val == flen and q == s:
                start = p
                break
        if start is None:
            continue
        # 二分/递增找最小可完整解析的长度
        fd = descriptor_pb2.FileDescriptorProto()
        ok = False
        # 先试大块：从 start 到下一个合理上界（256KB 内）
        lo, hi = flen + 2, min(len(data) - start, 512 * 1024)
        # 递增扫描太慢，改用：解析失败会抛 DecodeError；找最小 N 使解析成功
        # 用指数+二分
        def try_parse(n):
            f = descriptor_pb2.FileDescriptorProto()
            try:
                f.ParseFromString(data[start:start+n])
                return f
            except Exception:
                return None
        # 直接试整个 hi，若成功则说明尾部碰巧合法（不太可能），改为逐步逼近
        # 策略：从 flen+2 开始逐步扩大到首次成功
        f = None
        n = flen + 2
        step = 64
        while n <= hi:
            f = try_parse(n)
            if f is not None:
                break
            n += step
        if f is None:
            continue
        name = f.name
        if name in seen:
            continue
        seen.add(name)
        if name_filter and name_filter not in name:
            continue
        out = f'{out_dir}/{name.replace("/", "_")}.txt'
        with open(out, 'w', encoding='utf-8') as w:
            w.write(str(f))
        print(f'extracted: {name} ({len(f.message_type)} messages, {len(f.enum_type)} enums) -> {out}')

if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2], sys.argv[3] if len(sys.argv) > 3 else None)
