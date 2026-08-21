# 在 PE .text 中找指定字符串的 RIP 相对 xref（lea reg,[rip+disp]），打印引用点地址
import sys, pefile

def main(path, needle):
    pe = pefile.PE(path, fast_load=True)
    data = open(path, 'rb').read()
    off = data.find(needle)
    if off < 0:
        print('string not found'); return
    va = pe.OPTIONAL_HEADER.ImageBase + pe.get_rva_from_offset(off)
    print(f'string "{needle[:60]}..." at file off {off:#x} VA {va:#x}')
    # 找 .text 段
    text = None
    for s in pe.sections:
        if s.Name.startswith(b'.text'):
            text = s; break
    tva = pe.OPTIONAL_HEADER.ImageBase + text.VirtualAddress
    tdata = data[text.PointerToRawData: text.PointerToRawData + text.SizeOfRawData]
    # 扫描 lea reg, [rip+disp32]：48/4C 8D /r，modrm mod=00 rm=101
    found = []
    for i in range(len(tdata) - 7):
        b0 = tdata[i]
        if b0 not in (0x48, 0x4C):
            continue
        if tdata[i+1] != 0x8D:
            continue
        modrm = tdata[i+2]
        if (modrm & 0xC7) != 0x05:
            continue
        disp = int.from_bytes(tdata[i+3:i+7], 'little', signed=True)
        target = tva + i + 7 + disp
        if target == va:
            found.append(tva + i)
    for f in found:
        print(f'xref at VA {f:#x} (file off {pe.get_offset_from_rva(f - pe.OPTIONAL_HEADER.ImageBase):#x})')
    if not found:
        print('no lea xref found (string may be referenced via other means)')

if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2].encode())
