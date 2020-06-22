#include <iostream>
using namespace std;

int main(int argc, char *argv[]) {
    cout << "hello from c++" << endl;
    for (int i=0; i<0; ++i)
        cout << "  " << i << " : " << argv[i] << endl;
    return 0;
}
