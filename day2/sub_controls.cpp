#include <iostream>
#include <string> 
#include <fstream>


int main() {

    std::ifstream input;
    input.open("../input.txt");


    std::string direction = "";
    int cmd_value = 0;
    int horizontal_position = 0;
    int vertical_position = 0;


    while ( input >> direction >> cmd_value)
    {
    std::cout << direction << " " << cmd_value << std::endl;

        if(direction == "forward")
        {
            horizontal_position += cmd_value;
        }else if(direction=="up")
        {
            vertical_position += cmd_value;
        }else if(direction == "down")
        {
            vertical_position -= cmd_value;
        }else
        {
            std::cout<< "error!"<<std::endl;
            return 0;
        }


        

    }
        std::cout << horizontal_position << " "<< vertical_position<< " " << std::abs(vertical_position) * horizontal_position << std::endl;


    return 0;
}
