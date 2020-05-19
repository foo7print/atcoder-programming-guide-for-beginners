#include <bits/stdc++.h>
using namespace std;

int main()
{
  int A, B;
  cin >> A >> B;

  // ここにプログラムを追記
  cout << "A:";
  int i = 0;
  while (i < A)
  {
    cout << "]";
    i++;
  }

  // 改行
  cout << endl;

  cout << "B:";
  int j = 0;
  while (j < B)
  {
    cout << "]";
    j++;
  }

  // 問題の回答の最後は必ず改行していなければならない。無いとWAになる。
  cout << endl;
}
