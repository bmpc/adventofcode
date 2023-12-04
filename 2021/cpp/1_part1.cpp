#include <iostream>
#include <fstream>
#include <string>

int main(int argc, char const *argv[])
{
    int count = 0;

    std::ifstream file("input1.txt");
    std::string str; 

    if (!std::getline(file, str)) {
        std::cout << "No input available!\n";
    }
    int lastDepth = std::stoi(str);

    while (std::getline(file, str))
    {
        int depth = std::stoi(str);
        if (depth > lastDepth) {
            count++;
        }

        lastDepth = depth;
    }

    std::cout << "Result: " << count << "\n";

    file.close();
    return 0;
}