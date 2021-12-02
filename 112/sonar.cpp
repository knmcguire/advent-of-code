// Your First C++ Program

#include <iostream>
#include <string> 
#include <fstream>


int main() {

    std::ifstream input;
    input.open("../input.txt");
    bool first_line_hit = true;
    int current_measurement = 0;
    int previous_measurement = 0;
    int count = 0;

    for( std::string line; getline( input, line ); )
    {
        current_measurement = std::stoi( line );

        if(first_line_hit)
        {
            first_line_hit = false;
        }else if(current_measurement>previous_measurement)
        {
                count ++;
        }
        
        previous_measurement = current_measurement;

    }
    std::cout <<  count << " measurements are larger than it's previous measurement" << std::endl;
    return 0;
}



