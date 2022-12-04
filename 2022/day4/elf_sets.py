import re

fully_contained_sets_count = 0;
overlapping_sets_count = 0;

line_count = 0
with open("input.txt", "r") as file:
    for line in file:
        print(line_count)
        split_string = re.split('[,-]', line.replace("\n",""))
        print(split_string)
        if split_string[0] != split_string[1]:
            range1 = set(range(int(split_string[0]),int(split_string[1])+1))
        else:
            print(int(split_string[0]))
            range1 = {int(split_string[0])}


        if split_string[2] != split_string[3]:
            range2 = set(range(int(split_string[2]),int(split_string[3])+1))
        else:
            range2 = {int(split_string[2])}

        print(range1)
        print(range2)

        if range1.issubset(range2):
            fully_contained_sets_count += 1
        elif range2.issubset(range1):
            fully_contained_sets_count += 1

        if range1.intersection(range2):
            overlapping_sets_count += 1
            print('jeej!1')
        elif range2.intersection(range1):
            overlapping_sets_count += 1
            print('jeej!2')

        line_count+=1


print(fully_contained_sets_count)
print(overlapping_sets_count)