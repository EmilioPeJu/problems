import numpy as np
def get_val(domain):
    res = 0
    while not domain&1:
        res+=1
        domain>>=1
    return res


def get_val_if_only_one(domain):
    bit = domain&(-domain)
    if (domain - bit) == 0:
        return get_val(domain)
    return 0


def propagate_one(sudoku, domains, i, j):
    val = sudoku[i][j]
    for k in range(9):
        if sudoku[i][k] == 0:
            domains[i][k] &= ~(1<<val)
            if domains[i][k]==0:
                return False
            sudoku[i][k] = get_val_if_only_one(domains[i][k])
            if sudoku[i][k]:
                valid = propagate_one(sudoku, domains, i, k)
                if not valid:
                    return False
        elif k != j and sudoku[i][j] == sudoku[i][k]:
            return False

    for k in range(9):
        if sudoku[k][j] == 0:
            domains[k][j] &= ~(1<<val)
            if domains[k][j]==0:
                return False
            sudoku[k][j] = get_val_if_only_one(domains[k][j])
            if sudoku[k][j]:
                valid = propagate_one(sudoku, domains, k, j)
                if not valid:
                    return False
        elif k != i and sudoku[i][j] == sudoku[k][j]:
            return False

    start_i, start_j = i // 3 * 3, j // 3 * 3
    for cell_i in range(start_i, start_i + 3):
        for cell_j in  range(start_j, start_j + 3):
            if sudoku[cell_i][cell_j] == 0:
                domains[cell_i][cell_j] &= ~(1<<val)
                if domains[cell_i][cell_j] == 0:
                    return False
                sudoku[cell_i][cell_j] = get_val_if_only_one(
                    domains[cell_i][cell_j])
                if sudoku[cell_i][cell_j]:
                    valid = propagate_one(sudoku, domains, cell_i, cell_j)
                    if not valid:
                        return False
            elif not (cell_i == i and cell_j == j) and sudoku[i][j] == sudoku[cell_i][cell_j]:
                return False
    return True


def propagate_all(sudoku):
    domains = np.ones((9,9), dtype=np.int32)*(((1<<10)-1)-1)
    for i in range(9):
        for j in range(9):
            if sudoku[i][j] != 0:
                feasable = propagate_one(sudoku, domains, i, j)
                if not feasable:
                    return None
    return domains


backtracking_count = 0
sol_sudoku=None
solved = 0
def solve_sudoku(sudoku):
    global sol_sudoku
    global backtracking_count
    dirty_sudoku = sudoku.copy()
    domains = propagate_all(dirty_sudoku)
    if domains is None: # not feasable
        return False
    if sol_sudoku is not None:
        return True

    if 0 not in dirty_sudoku: # solved
        sol_sudoku = dirty_sudoku
        return True

    i, j = select_var(dirty_sudoku, domains)
    for val in get_vals(dirty_sudoku, domains, i,j):
        backtracking_count += 1
        dirty_sudoku[i][j] = val
        feasable = solve_sudoku(dirty_sudoku)
        if feasable:
            return True

def select_var(sudoku, domains):
    # choose one with fewer options (less backtracking)
    def count_bits(x):
        res = 0
        while x:
            if x&1:
                res += 1
            x>>=1
        return res

    min_len = 10
    pos = None
    for i in range(9):
        for j in range(9):
            if sudoku[i][j] == 0:
                nbits = count_bits(domains[i][j])
                if min_len > nbits:
                    pos = (i, j)
                    min_len = nbits

    return pos


def get_vals(sudoku, domains, i, j):
    # iterate possible (sorted ) values
    for k in range(1, 10):
        if domains[i][j] & (1<<k):
            yield k


input_file = "p096_sudoku.txt"
total_res = 0
data = open(input_file, 'r').read().split('\n')

for index in range(0, len(data), 10):
    print("Sudoku {}".format(data[index]))
    sol_sudoku = None
    sudoku = [[int(j) for j in i] for i in data[index+1:index+10]]

    sudoku = np.array(sudoku, dtype=np.int32)
    print(sudoku)

    feasable = solve_sudoku(sudoku)
    if sol_sudoku is not None:
        print(sol_sudoku)
        print("backtracking count: {}".format(backtracking_count))
        total_res += int(sol_sudoku[0][2]) + int(sol_sudoku[0][1])*10 + \
            int(sol_sudoku[0][0])*100
        solved += 1
    else:
        print("Not feasable")

print("Solved {} result: {}".format(solved, total_res))
