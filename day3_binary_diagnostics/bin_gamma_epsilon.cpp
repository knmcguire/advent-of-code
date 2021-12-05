#include <iostream>
#include <string> 
#include <fstream>
#include <cmath>

using namespace std;

int main (int argc, char** argv)
{

    if (argc!=3)
    {
    cout << "ERROR. Usage is ./bin_gamma_epsilon filename bin_size"<<endl;
    return 1;
    }

    std::ifstream input;
    input.open(argv[1], ios::in | ios::binary);
    long long int number;

    int dec= 0;
    int i = 0;
    int rem;
    int const bin_num_size = stoi(argv[2]);
    int count_array[bin_num_size] = {0};
    int line_count = 0;


    while ( input >> number)
    {
        cout<<number;

        while(number!=0)
        {
            rem = number%10;
            number /=10;
            if (rem)
            {
                count_array[i]++;
            }
            i++;
        }
        cout<<endl;
        i = 0;
        dec = 0;
        line_count ++;
    }

    int num_gamma = 0;
    int num_epsilon = 0;

    for(int it=0; it<bin_num_size; it++)
    {
        //cout<<count_array[it];
        if(count_array[it]>line_count/2)
        {
            cout<<1;
            num_gamma += pow(2, it);
        }
        else{
           cout<<0;
           num_epsilon += pow(2, it);

        }
    }
    cout<<"="<< num_gamma << " and it's binary inverse is: "<< num_epsilon <<endl;
    cout<<"answer is "<< num_gamma * num_epsilon <<endl;

    return 0;
}