#include<iostream>
#include <vector>
using namespace std;

int main() {
    std::vector<int> a({1,2,3,4});
    int &b = a[2];
    a.clear();
    a.shrink_to_fit();
    cout<<"B after clearing: "<<b<<endl;
}
