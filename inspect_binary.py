import struct, json, sys

path = sys.argv[1] if len(sys.argv) > 1 else "knowledge.atlas"

with open(path, "rb") as f:
    data = f.read()

magic = data[:6]
print(f"Magic: {magic}")

schema_major, schema_minor = struct.unpack_from("<HH", data, 6)
flags, = struct.unpack_from("<I", data, 10)
section_count, = struct.unpack_from("<H", data, 14)
toc_offset, = struct.unpack_from("<Q", data, 16)

print(f"Schema: {schema_major}.{schema_minor}")
print(f"Flags: {flags:#x}")
print(f"Sections: {section_count}")
print(f"TOC offset: {toc_offset}")

off = toc_offset
sections = []
for _ in range(section_count):
    name_end = data[off:].index(0)
    name = data[off:off+name_end].decode()
    off += name_end + 1
    offset = struct.unpack_from("<Q", data, off)[0]
    off += 8
    length = struct.unpack_from("<Q", data, off)[0]
    off += 8
    checksum = data[off:off+32]
    off += 32
    sections.append((name, offset, length))

for name, offset, length in sections:
    print(f"  {name:20s} offset={offset:8d} length={length}")