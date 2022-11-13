from PIL import Image
import math


def copy_area(source_pixels, target_pixels, source_x: int, source_y: int, width: int, height: int, destination_x: int, destination_y: int):
    for x in range(width):
        for y in range(height):
            target_pixel = list(target_pixels[destination_x + x, destination_y + y])
            source_pixel = list(source_pixels[source_x + x, source_y + y])

            if source_pixel == [0, 255, 0, 255]:
                target_pixel = [0, 0, 0, 0]
            else:
                for i in range(3):
                    target_pixel[i] = int(source_pixel[3] / 255 * source_pixel[i] + (255 - source_pixel[3]) / 255 * target_pixel[i])

            target_pixels[destination_x + x, destination_y + y] = tuple(target_pixel)


def generate_block_texture(input_file_path: str, output_file_path: str):
    output_image = Image.new("RGBA", (8, 128), (255, 255, 255, 255))
    output_pixels = output_image.load()

    input_image = Image.open(input_file_path)
    input_pixels = input_image.load()
    _, input_height = input_image.size

    for y in range(0, 128, 8):
        copy_area(input_pixels, output_pixels, 0, 0, 8, 8, 0, y)

    for texture_y in range(8, input_height - 31, 32):
        for i in range(4):
            for y in range(0, 16, 1):
                if int(y / math.pow(2, i)) % 2 == 0:
                    copy_area(input_pixels, output_pixels, 0, i * 8 + texture_y, 8, 8, 0, y * 8)

    for i in range(16):
        if int(i / 8) % 2 == 0 and i % 2 == 0:
            output_pixels[0, i * 8] = (0, 0, 0, 0)
        if i % 2 == 0 and int(i / 2) % 2 == 0:
            output_pixels[7, i * 8] = (0, 0, 0, 0)
        if int(i / 2) % 2 == 0 and int(i / 4) % 2 == 0:
            output_pixels[7, i * 8 + 7] = (0, 0, 0, 0)
        if int(i / 4) % 2 == 0 and int(i / 8) % 2 == 0:
            output_pixels[0, i * 8 + 7] = (0, 0, 0, 0)

    output_image.save(output_file_path)
