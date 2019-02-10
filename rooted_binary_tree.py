#!/usr/bin/env python
from collections import deque

class Element(object):
    def __init__(self, key, left=None, right=None):
        self.key = key
        self.left = left
        self.right = right
        self.left_count = 0
        self.right_count = 0
        self.count = 0
        self.offset = 0


def build_tree(preorder, inorder):
    preindex = [0]
    return _build_tree(preorder, inorder, preindex)


def _build_tree(preorder, inorder, preindex):
    current = preorder[preindex[0]]
    tree = Element(current)
    inindex = inorder.find(current)
    preindex[0] += 1
    if inindex > 0:
        tree.left = _build_tree(preorder, inorder[:inindex], preindex)
    if inindex + 1 < len(inorder):
        tree.right = _build_tree(preorder, inorder[inindex + 1:], preindex)
    return tree


def count_tree(tree, level):
    left_count = 0
    right_count = 0
    if tree.left:
        count_tree(tree.left, level + 1)
        left_count = tree.left.count
    if tree.right:
        count_tree(tree.right, level + 1)
        right_count = tree.right.count
    tree.count = left_count + right_count + 1
    tree.left_count = left_count
    tree.right_count = right_count
    tree.level = level

def is_right(tree, parent):
    return tree is parent.right

def is_left(tree, parent):
    return tree is parent.left

def fill_screen(tree, parent, screen):
    if is_left(tree, parent):
        offset = parent.offset - tree.right_count - 1
    elif is_right(tree, parent):
        offset = parent.offset + tree.left_count + 1
    else:
        offset = tree.left_count
    tree.offset = offset
    screen[tree.level][offset] = tree.key
    if tree.left:
        fill_screen(tree.left, tree, screen)
    if tree.right:
        fill_screen(tree.right, tree, screen)


def main():
    while True:
        screen = [[' ' for _ in range(60)] for _ in range(60)]
        try:
            inorder = input().strip()
            preorder = input().strip()
        except EOFError:
            break
        tree = build_tree(preorder, inorder)
        count_tree(tree, 0)
        fill_screen(tree, tree, screen)
        for line in screen:
            if "".join(line).strip():
                print("".join(line).rstrip())
            else:
                break


if __name__ == "__main__":
    main()