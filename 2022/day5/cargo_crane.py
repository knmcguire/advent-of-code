import re

# Input a list of strings where alphacharacters are within delimiters [ ]¨

def parse_blocks_into_grid(input_blocks):
    # replace three empty consecutive characthers with "[ ]"
    print(len(input_blocks))
    block_string_list =  ["" for i in range(len(input_blocks[0]))] 
    for i in range(len(input_blocks)):
        line = input_blocks[len(input_blocks)-1-i]
        chars = [c for c in line]
        n = 0
        for char in chars:
            if char != "0":
                block_string_list[n] += char
            n += 1
    return block_string_list

def get_number_after_substring(substring, string):
    # get the number after a substring
    match = re.search(substring + '(\d+)', string)
    if match:
        print(match.group(1))

    return int(match.group(1))
    
def change_blocks_based_on_commands(block_string_list, commands, part_two=False):
    # change the blocks based on the commands
    #amount_of_blocks = int(commands[commands.index("move ") + len("move ")])
    amount_of_blocks = get_number_after_substring("move ", commands)
    first_postion = int(commands[commands.index("from ") + len("from ")])-1
    second_position = int(commands[commands.index("to ") + len("to ")])-1
    print(block_string_list, amount_of_blocks,first_postion,second_position)
    if part_two == False:
        for it in range(amount_of_blocks):
            assert len(block_string_list[first_postion]) > 0
            temp_char = block_string_list[first_postion][-1]
            block_string_list[second_position] += temp_char
            block_string_list[first_postion] = block_string_list[first_postion][:-1]
    else:
        temp_chars = block_string_list[first_postion][-amount_of_blocks:]
        block_string_list[second_position] += temp_chars
        block_string_list[first_postion] = block_string_list[first_postion][:-amount_of_blocks]

    return block_string_list
    

def get_last_characters(block_state_list):
    # get the last characters of the blocks
    last_chars = []
    for block in block_state_list:
        if len(block) > 0:
            last_chars.append(block[-1])
        else:
            last_chars.append(" ")
    last_char_string = "".join(last_chars)
    return last_char_string



if __name__ == '__main__':

    block_init = []
    it = 0
    part_two = True
    with open("input.txt", "r") as file:

        for line in file:
            print(it)
            it += 1
            chars = [c for c in line]
            if chars[0]=="m":
                print('character is command')
                block_state_list = change_blocks_based_on_commands(block_state_list, line,part_two=part_two)
            elif chars[0]=='\n':
                print('character is linebreak')
                print(block_init)
                block_state_list = parse_blocks_into_grid(block_init)
                print(block_state_list)
            elif chars[0]==' ' and chars[1].isdigit():
                print('character are row numbers')
            else:
                print('character are blocks')
                # Replace 3 empty characters with '[ ]'
                line=line.replace("    [", "[0] [")
                line=line.replace("]    ", "] [0]")
                line=line.replace("    ", "[0] ")

                line=line.replace("] [ ", "")
                line=line.replace("[", "")
                line=line.replace("]", "")
                line=line.replace(" ", "")
                line=line.replace("\n", "")

                block_init.append(line)

        print(block_state_list)
        print(get_last_characters(block_state_list))





        
