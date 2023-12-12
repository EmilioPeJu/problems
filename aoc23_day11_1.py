#!/usr/bin/env python

def read_image():
    res = []
    while True:
        try:
            res.append(input().strip())
        except EOFError:
            break

    return res


def is_row_empty(image, ri):
    for i in range(len(image[0])):
        if image[ri][i] != '.':
            return False

    return True


def is_col_empty(image, ci):
    for i in range(len(image)):
        if image[i][ci] != '.':
            return False

    return True


def get_empty_sum(image):
    rows = [0] * len(image)
    cols = [0] * len(image[0])
    rows[0] = int(is_row_empty(image, 0))
    for ri in range(1, len(image)):
        rows[ri] = rows[ri - 1] + int(is_row_empty(image, ri))

    cols[0] = int(is_col_empty(image, 0))
    for ci in range(1, len(image[0])):
        cols[ci] = cols[ci - 1] + int(is_col_empty(image, ci))

    return rows, cols


def get_all_galaxies(image):
    res = []
    for i in range(len(image)):
        for j in range(len(image[0])):
            if image[i][j] == '#':
                res.append((i, j))

    return res


def galaxies_distance(gal1, gal2, empty_sum_r, empty_sum_c):
    a = min(gal2[0], gal1[0])
    b = max(gal2[0], gal1[0])
    dist_r = b - a
    dist_r += empty_sum_r[b] - empty_sum_r[a]
    a = min(gal2[1], gal1[1])
    b = max(gal2[1], gal1[1])
    dist_c = b - a
    dist_c += empty_sum_c[b] - empty_sum_c[a]
    return dist_r + dist_c


def main():
    image = read_image()
    galaxies = get_all_galaxies(image)
    empty_sum_r, empty_sum_c = get_empty_sum(image)
    res = 0
    for i in range(len(galaxies)):
        for j in range(i + 1, len(galaxies)):
            distance = galaxies_distance(galaxies[i], galaxies[j], empty_sum_r,
                                         empty_sum_c)
            res += distance

    print(res)


if __name__ == '__main__':
    main()
