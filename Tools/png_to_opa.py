# opa - ordered pixel array
import sys
import cv2


def convert(input_file: str, output_file: str):
    input_img = cv2.imread(input_file, cv2.IMREAD_UNCHANGED)

    height, width, channels = input_img.shape

    result_data = [0] * 8

    result_data[0] = (width >> 0) & 0xFF
    result_data[1] = (width >> 8) & 0xFF
    result_data[2] = (width >> 16) & 0xFF
    result_data[3] = (width >> 24) & 0xFF
    result_data[4] = (height >> 0) & 0xFF
    result_data[5] = (height >> 8) & 0xFF
    result_data[6] = (height >> 16) & 0xFF
    result_data[7] = (height >> 24) & 0xFF

    for y in range(height):
        for x in range(width):
            for i in range(4):
                if i < channels:
                    result_data.append(input_img[height - y - 1, x, i])
                else:
                    result_data.append(255)

    with open(output_file, "wb") as output_img:
        output_img.write(bytes(result_data))


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage png_to_opa.py [input_file.png] [output_file.opa]")
    else:
        convert(sys.argv[1], sys.argv[2])
