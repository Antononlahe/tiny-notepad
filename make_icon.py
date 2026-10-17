from pathlib import Path
import struct

SIZE = 32
TRANSPARENT = (0, 0, 0, 0)
pixels = [[TRANSPARENT for _ in range(SIZE)] for _ in range(SIZE)]


def fill_rect(left, top, right, bottom, color):
    for y in range(top, bottom + 1):
        for x in range(left, right + 1):
            pixels[y][x] = color


def line(x0, y0, x1, y1, color):
    dx = abs(x1 - x0)
    dy = -abs(y1 - y0)
    sx = 1 if x0 < x1 else -1
    sy = 1 if y0 < y1 else -1
    error = dx + dy
    while True:
        pixels[y0][x0] = color
        if x0 == x1 and y0 == y1:
            return
        twice_error = 2 * error
        if twice_error >= dy:
            error += dy
            x0 += sx
        if twice_error <= dx:
            error += dx
            y0 += sy


navy = (0, 0, 128, 255)
paper = (192, 192, 192, 255)
light = (255, 255, 255, 255)
blue = (0, 0, 255, 255)

# A tiny lined note page; Windows adds the shortcut arrow to the desktop link.
fill_rect(8, 2, 26, 28, navy)
fill_rect(9, 3, 25, 27, paper)
fill_rect(21, 3, 25, 7, light)
line(21, 3, 21, 7, navy)
line(21, 7, 25, 7, navy)
fill_rect(11, 9, 22, 12, navy)
for y in (15, 18, 21, 24):
    fill_rect(11, y, 23 if y == 15 else 21, y, navy)

header = struct.pack("<IIIHHIIIIII", 40, SIZE, SIZE * 2, 1, 32, 0, SIZE * SIZE * 4, 0, 0, 0, 0)
xor_bitmap = bytearray()
and_bitmap = bytearray()
mask_stride = ((SIZE + 31) // 32) * 4
for y in range(SIZE - 1, -1, -1):
    mask_row = bytearray(mask_stride)
    for x in range(SIZE):
        red, green, blue_channel, alpha = pixels[y][x]
        xor_bitmap.extend((blue_channel, green, red, alpha))
        if alpha == 0:
            mask_row[x // 8] |= 1 << (7 - x % 8)
    and_bitmap.extend(mask_row)

image = header + xor_bitmap + and_bitmap
directory = struct.pack("<HHH", 0, 1, 1)
entry = struct.pack("<BBBBHHII", SIZE, SIZE, 0, 0, 1, 32, len(image), 22)
Path(__file__).with_name("shortcut-notepad.ico").write_bytes(directory + entry + image)
