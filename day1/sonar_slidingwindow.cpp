// Your First C++ Program

#include <iostream>
#include <string> 
#include <fstream>


int main() {

    std::ifstream input;
    input.open("../input.txt");
    bool first_line_hit = true;
    int line_count = 0;
    int current_measurement = 0;
    int previous_measurement = 0;
    int prevprev_measurement = 0;
    int prev_sliding_avg = 0;
    int current_sliding_avg = 0;

    int count = 0;

    for( std::string line; getline( input, line ); )
    {
        current_measurement = std::stoi( line );
        
        current_sliding_avg = current_measurement + previous_measurement + prevprev_measurement;
        line_count++;

        if(line_count>3 && current_sliding_avg > prev_sliding_avg)
        {

                count++;
        }

        
        prev_sliding_avg = current_sliding_avg;
        prevprev_measurement = previous_measurement;
        previous_measurement = current_measurement;

    }
    std::cout <<  count << " measurements are larger than it's previous measurement" << std::endl;
    return 0;
}



