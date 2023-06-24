import cv2
import numpy as np
import easyocr

reader = easyocr.Reader(['en'])

# read the image
img = cv2.imread('sudoku.jpg')

# preprocess the image
gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
blur = cv2.GaussianBlur(gray, (5, 5), 0)
thresh = cv2.adaptiveThreshold(blur, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY_INV, 11, 2)

# extract the cells
contours, hierarchy = cv2.findContours(thresh, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
cells = []
for i in range(len(contours)):
    if hierarchy[0][i][3] == -1:
        x, y, w, h = cv2.boundingRect(contours[i])
        cell = thresh[y:y+h, x:x+w]
        cell = cv2.resize(cell, (28, 28))
        cells.append(cell)

# recognize the digits using EasyOCR
digits = []
for cell in cells:
    result = reader.readtext(cell)
    if len(result) > 0:
        digit = result[0][1]
        if digit.isdigit():
            digits.append(int(digit))
        else:
            digits.append(0)
    else:
        digits.append(0)
