import numpy as np
from itertools import combinations 

with open('data/day_11.dat') as f:
    galaxies = [[line.rstrip()] for line in f]

gal_np = np.array(galaxies)

# Convert to numbers 
def covert_to_numbers(gal_np):
    gal_np_nums = np.zeros(shape = (gal_np.shape[0], len(gal_np[0][0])))
    gal_num = 1

    for row_idx in range(gal_np.shape[0]):
        for col_idx in range(len(gal_np[0][0])):
            if gal_np[row_idx][0][col_idx] == ".":
                gal_np_nums[row_idx][col_idx] = 0
            if gal_np[row_idx][0][col_idx] == "#":
                gal_np_nums[row_idx][col_idx] = gal_num
                gal_num += 1
    return gal_np_nums, gal_num

def find_empty_cols(gal_np):
    where_empty_cols = [col_idx for col_idx in range(gal_np.shape[1]) if gal_np[:,col_idx].sum() == 0]
    return where_empty_cols

def find_empty_rows(gal_np):
    where_empty_cols = [row_idx for row_idx in range(gal_np.shape[0]) if gal_np[row_idx].sum() == 0]
    return where_empty_cols


# Find the coordinates of a galaxy
def find_galaxy_coords(gal_np) -> list[list[int]]:
    result = []
    for row_idx in range(gal_np.shape[0]):
        for col_idx in range(gal_np.shape[1]):
            if gal_np[row_idx][col_idx] != '.':
                result.append([row_idx, col_idx])
    return result

# Calculate Manhattan distance between two points
    # |x1 - x2| + |y1 - y2|
def calc_manhattan_dist(x1, x2, y1, y2):
    return abs(x1 - x2) + abs(y1 - y2)

def main():
    # print(gal_np)
    # gal_np_conv, num_gal = covert_to_numbers(gal_np)
    # print(gal_np_conv)
    # gal_perms = [i for i in list(combinations(range(num_gal), 2))]
    # print(len(gal_perms))

    # Find empty rows, cols index
    empty_cols_idx = find_empty_cols(gal_np)
    empty_rows_idx = find_empty_rows(gal_np)

    sum_path_between_galaxies = 0
    galaxies = find_galaxy_coords(gal_np) 
    for pair in list(combinations(galaxies, 2)):
        x1, y1 = pair[0]
        print(x1, y1)

        x2, y2 = pair[1]
        print(x2, y2)

        # Find min and max of rows and cols
        min_row = min(x1, x2)
        max_row = max(x1, x2)

        min_col = min(y1, y2)
        max_col = max(y1, y2)

        # Find the Manhattan distance between each pair
        distance_pair = calc_manhattan_dist(x1, x2, y1, y2)

        # Count number of empty rows between pair
        for row_idx in empty_rows_idx:
            if min_row < row_idx and row_idx < max_row:
                distance_pair += 1

        # Count number of empty cols between pair
        for col_idx in empty_cols_idx:
            if min_col < col_idx and col_idx < max_col:
                distance_pair += 1


        # Add to sum of path between galaxies
        sum_path_between_galaxies += distance_pair
    print(sum_path_between_galaxies)

if __name__ == "__main__":
    main()


     # # Count the empty columns 
# def count_empty_cols(subset_gal_np):
#     num_empty_cols = 0
#     for col_idx in range(subset_gal_np.shape[1]):
#         if subset_gal_np[:,col_idx].sum() == 0:
#             num_empty_cols += 1
#     return num_empty_cols

# # Count the empty rows in a subset of the array
# def count_empty_rows(subset_gal_np):
#     num_empty_rows = 0
#     for row_idx in range(subset_gal_np.shape[0]):
#         if subset_gal_np[row_idx].sum() == 0:
#             num_empty_rows += 1
#     return num_empty_rows
   

# Count number of empty cols between pair

# # Create subset of array between each pair
# gal_sub_rows = gal_np_conv[x1:x2+1, :]

# # Accounts for if first number is to the right of the second number in the cols
#     # Never true for the rows due to ordering of combinations, but could do the same thing
# if y1 < y2:
#     gal_sub_cols = gal_np_conv[:, y1:y2+1]
# else:
#     gal_sub_cols = gal_np_conv[:, y2:y1+1]
# # print(gal_sub_rows)
# # print(gal_sub_cols)

# # Increment row if empty rows between them 
# distance_pair += count_empty_rows(gal_sub_rows)

# # Increment column if empty columns between them
# distance_pair += count_empty_cols(gal_sub_cols)
# print(distance_pair)

    
    
    




