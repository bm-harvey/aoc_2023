import time
def calculate_elapsed_time(start_time):
    return time.time() - start_time
start_time = time.time() 

def distance_calc(time,record_distance):
    options=0
    for ii in range(1,time,1):
        if(ii*(time-ii)> record_distance):
            options=options +1
    return(options)

def options(time_array, distance_array):
    time_indices = [(index,time) for index, time in enumerate(time_array)]
    options_sum=1
    for index, time in time_indices:
        options_sum=options_sum*distance_calc(time_array[index],distance_array[index])     
    return(options_sum)

def clever_math(my_array):
    sum_value =0
    for index, value in enumerate(my_array):
        if index == 0:
            sum_value= my_array[index]
        elif index > 0:
                sum_value= sum_value * 10**int(len(str(my_array[index]))) + my_array[index]
    return(sum_value)

with open('data/day_6.dat', 'r') as file:
    race_time=[]
    race_distance=[]
    waste=[]
    for line in file:
        elements = line.split()  # Split line into elements based on spaces
        for element in elements:
            if element.isdigit() and int(element)<100:
                race_time.append(int(element))
                #print(f"time:", race_time[-1])
            elif line[2] and element.isdigit():
                race_distance.append(int(element))
                #print(f"distance:", race_distance[-1])

final_value = options(race_time,race_distance)
clever_math(race_time)
part2 = distance_calc(clever_math(race_time),clever_math(race_distance))

elapsed_time = calculate_elapsed_time(start_time)
print(f"The code has been running for {elapsed_time} seconds.")
