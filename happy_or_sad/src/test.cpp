#include <iostream>
using namespace std;

int main()
{
    int squares[4][4] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0 ,0 ,0 ,0 };

    for (int i = 0; i < 4; i++){
        for (int e = 0; e < 4; e++){
            int num;
            cin >> num;
            num = squares[i][e];
        }
    }
    int n = squares[0][0] + squares [0][1] + squares [0][2] + squares [0][3];

    if ((squares[1][0] + squares [1][1] + squares [1][2] + squares [1][3]) == n ) {
        if ((squares[2][0] + squares [2][1] + squares [2][2] + squares [2][3]) == n ) {
            if ((squares[3][0] + squares [3][1] + squares [3][2] + squares [3][3]) == n ) {
                if ((squares[0][0] + squares [1][0] + squares [2][0] + squares [3][0]) == n ) {
                    if ((squares[0][1] + squares [1][1] + squares [2][1] + squares [3][1]) == n ) {
                        if ((squares[0][2] + squares [1][2] + squares [2][2] + squares [3][2]) == n ) {
                            if ((squares[0][3] + squares [1][3] + squares [2][3] + squares [3][3]) == n ) {
                                if ((squares[0][0] + squares [1][1] + squares [2][2] + squares [3][3]) == n ) {
                                    cout << "magic" << endl;
                                    return 0;
                                }
                            }
                        }
                    }
                }
            }
        }
    } 

    cout << "not magic" << endl;


    return 0;
}