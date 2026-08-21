# 从指定文件偏移向前 back 字节开始线性反汇编到偏移处（x64）
import sys, pefile
from capstone import *

def main(path, off_hex, back_hex):
    pe = pefile.PE(path, fast_load=True)
    data = open(path, 'rb').read()
    off = int(off_hex, 16); back = int(back_hex, 16)
    start = max(0, off - back)
    blob = data[start:off + 0x40]
    base = pe.OPTIONAL_HEADER.ImageBase + pe.get_rva_from_offset(start)
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True
    for ins in md.disasm(blob, base):
        print(f'{ins.address:#x}  {ins.mnemonic:10s} {ins.op_str}')

if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2], sys.argv[3])
