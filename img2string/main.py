import cv2
import pytesseract
import numpy as np

# read the image
img = cv2.imread('sudoku.jpg')

# preprocess the image
gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
blur = cv2.GaussianBlur(gray, (5, 5), 0)
# thresh = cv2.adaptiveThreshold(blur, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY_INV, 11, 2)
thresh = cv2.adaptiveThreshold(blur, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY, 21, 7)
cv2.imwrite('thresh.jpg', thresh)



def extract_cells(image):
    # Convert the image to grayscale
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

    # Apply Gaussian blur to reduce noise
    blur = cv2.GaussianBlur(gray, (5, 5), 0)

    # Apply Canny edge detection to find the grid lines
    edges = cv2.Canny(blur, 50, 150, apertureSize=3)

    # Find the Hough lines in the edge image
    lines = cv2.HoughLinesP(edges, 1, np.pi/180, 200, minLineLength=100, maxLineGap=10)

    # Find the longest line segment for each of the four sides of the grid
    left_line = max(lines, key=lambda line: line[0][0])[:, 0] if lines is not None else [0, 0, 0, image.shape[0]]
    right_line = max(lines, key=lambda line: line[0][2])[:, 0] if lines is not None else [image.shape[1], 0, image.shape[1], image.shape[0]]
    top_line = max(lines, key=lambda line: line[0][1])[:, 0] if lines is not None else [0, 0, image.shape[1], 0]
    bottom_line = max(lines, key=lambda line: line[0][3])[:, 0] if lines is not None else [0, image.shape[0], image.shape[1], image.shape[0]]

    # Calculate the corners of the grid based on the line segments
    top_left = intersection(top_line, left_line)
    top_right = intersection(top_line, right_line)
    bottom_left = intersection(bottom_line, left_line)
    bottom_right = intersection(bottom_line, right_line)

    # Calculate the width and height of each cell
    grid_width = bottom_right[0] - top_left[0]
    grid_height = bottom_right[1] - top_left[1]
    cell_width = int(grid_width / 9)
    cell_height = int(grid_height / 9)

    # Extract each cell as a separate image
    cells = []
    for i in range(9):
        for j in range(9):
            x = top_left[0] + j * cell_width
            y = top_left[1] + i * cell_height
            cell = gray[y:y+cell_height, x:x+cell_width]
            cells.append(cell)

    return cells

def intersection(line1, line2):
    # Calculate the intersection point of two lines
    x1, y1, x2, y2 = line1
    x3, y3, x4, y4 = line2
    denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1)
    if denom == 0:
        return [0, 0]
    else:
        ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denom
        return [int(x1 + ua * (x2 - x1)), int(y1 + ua * (y2 - y1))]

cells = extract_cells(thresh)


# print(cells)
print(len(cells))
# recognize the digits using pytesseract
digits = []

def recognize_digit(cell):
    # Preprocess the cell image
    cell = cv2.GaussianBlur(cell, (5, 5), 0)
    cell = cv2.threshold(cell, 0, 255, cv2.THRESH_OTSU)[1]
    kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (3, 3))
    cell = cv2.erode(cell, kernel, iterations=1)

    # Recognize the digit using Tesseract
    config = '--psm 10 outputbase digits'
    digit = pytesseract.image_to_string(cell, config=config)

    # Convert the recognized digit to an integer, or return 0 if no digit was recognized
    try:
        digit = int(digit)
        return digit
    except ValueError:
        return 0

    
for cell in cells:
    
    digit = recognize_digit(cell)
    digits.append(digit)

def print_puzzle(puzzle_string):
    for i in range(9):
        for j in range(9):
            print(puzzle_string[i*9 + j], end=' ')
            if j % 3 == 2 and j < 8:
                print('|', end=' ')
        print()
        if i % 3 == 2 and i < 8:
            print('-' * 21)

# output the puzzle string
puzzle_string = ''.join(str(digit) for digit in digits)
print(len(digits), digits)
print(len(puzzle_string), puzzle_string)

    
print_puzzle(puzzle_string)