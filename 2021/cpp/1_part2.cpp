#include <iostream>
#include <fstream>
#include <string>
#include <cstddef>

#define DEBUG 1

int read_line_number(std::ifstream &file) {
    std::string str;
    if (std::getline(file, str)) {
        return std::stoi(str);
    }

    return -1;
}

constexpr char next_char(char c) {
    if (c == 'Z') return 'A';

    return c + 1;
}

int main(int argc, char const *argv[])
{
    int count = 0;

    std::ifstream file("input1.txt");

    int tail = read_line_number(file);
    int middle = read_line_number(file);
    int head = read_line_number(file);
    int sum = tail + middle + head;
    char c = 'A';
    int new_head = 0;

    std::cout << c << ": " << sum << " " << "(N/A - no previous sum)" << "\n";

    while((new_head = read_line_number(file)) > 0) {
        tail = middle;
        middle = head;
        head = new_head;

        int new_sum = tail + middle + head;

        int increased = new_sum - sum;

        #ifdef DEBUG
        std::string status;
        if (increased == 0) {
            status = "(no change)";
        } else {
            status = increased < 0 ? "(decreased)" : "(increased)";
        }
        c = next_char(c);
        std::cout << c << ": " << new_sum << " " << status << "\n";
        #endif

        if (increased > 0) {
            count++;
        }

        sum = new_sum;
    }

    std::cout << "Result: " << count << "\n";

    file.close();
    return 0;
}