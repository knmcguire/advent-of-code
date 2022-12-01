#include <iostream>
#include <string> 
#include <fstream>
#include <cmath>
#include <vector>
#include <bitset>


using namespace std;

int BinToDec(int bin_num)
{
    int i = 0;
    int temp;
    int dec_num=0;
    while(bin_num!=0)
        {
            temp = bin_num%10;
            bin_num /=10;
            dec_num += temp*pow(2, i);
            i++;
        }
    return dec_num;
}

int main (int argc, char** argv)
{
    const int bin_num_size = stoi(argv[2]);

    if (argc!=3)
    {
        cout << "ERROR. Usage is ./bin_gamma_epsilon filename bin_size"<<endl;
        return 1;
    }else if(bin_num_size>12)
    {
        cout << "ERROR. can not use an bin size larger than 12"<<endl;
        return 1;
    }


    std::ifstream input;
    input.open(argv[1]);
    string strnumber;

    int line_count = 0;
    const int max_bit_size = 12;

    vector<bitset<max_bit_size> > number_array;

    while ( input >> strnumber)
    {
        number_array.push_back(BinToDec(stol(strnumber)));
    }

    int array_size_mostcommon = number_array.size();
        int array_size_leastcommon = number_array.size();

    bitset<max_bit_size> temp_num;
    int temp_decimal;

    int count_mostcommon = 0;
    int count_leastcommon = 0;

    bitset<max_bit_size> flag_mostcommon(0);
    bitset<max_bit_size> flag_leastcommon(0);
    bitset<max_bit_size> mask_check(0);


    for(int it = bin_num_size-1; it > -2; it--)
    {
        for (int ita = 0; ita< number_array.size();ita++)
        {
            temp_num = number_array[ita];
            if(it<0){
                // do something extra
            }else{
                if((mask_check&temp_num) == flag_mostcommon) { if( temp_num.test(it)){ count_mostcommon++;}}
                else { array_size_mostcommon--; }

                if((mask_check&temp_num) == flag_leastcommon){ if( temp_num.test(it)){ count_leastcommon++; }}
                else{ array_size_leastcommon--; }
            }
        }

        if (it>=0)
        {
            if (array_size_mostcommon == 0 ) {}// don't do anything
            else if(array_size_mostcommon==1){(count_mostcommon) ? flag_mostcommon.set(it, 1) : flag_mostcommon.set(it, 0); }
            else if(array_size_mostcommon == count_mostcommon){ flag_mostcommon.set(it, 1);}
            else if(count_mostcommon==0 && array_size_leastcommon>0){ flag_mostcommon.set(it, 0);}
            else { (count_mostcommon*2>=array_size_mostcommon) ? flag_mostcommon.set(it, 1) : flag_mostcommon.set(it, 0);}

            if (array_size_leastcommon == 0) { } // don't do anything
            else if(array_size_leastcommon==1){(count_leastcommon) ? flag_leastcommon.set(it, 1) : flag_leastcommon.set(it, 0); }
            else if(array_size_leastcommon == count_leastcommon){ flag_leastcommon.set(it, 1);}
            else if(count_leastcommon==0 && array_size_leastcommon>0){ flag_leastcommon.set(it, 0);}
            else {(2*count_leastcommon>=array_size_leastcommon) ? flag_leastcommon.set(it, 0) : flag_leastcommon.set(it, 1);}
        
            //mask_check.set(it-1,0);
            mask_check.set(it,1);
        }
        
        count_mostcommon = 0;
        count_leastcommon = 0;
        array_size_leastcommon = number_array.size();;
        array_size_mostcommon = number_array.size();;

    }
        cout<<"final flags : "<< flag_mostcommon<<" * "<<flag_leastcommon<<endl;
        cout<<"final answer : "<< flag_mostcommon.to_ulong()<<" * "<<flag_leastcommon.to_ulong();
        cout<<" = "<<flag_leastcommon.to_ulong()* flag_mostcommon.to_ulong()<<endl;

    return 0;
}