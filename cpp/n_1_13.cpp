#include <bits/stdc++.h>
using namespace std;

int main()
{
  int N;
  cin >> N;

  // N要素の配列
  vector<int> A(N);

  // 入力処理
  for (int i = 0; i < N; i++) {
    cin >> A.at(i);
  }

  // 合計点
  int sum = 0;

  // 合計点を計算
  for (int i = 0; i < N; i++) {
    sum += A.at(i);
  }

  // 平均点
  int mean = sum / N;

  // 平均点から何点離れているかを計算して出力
  for (int i = 0; i < N; i++) {
    // 負の数を出力しないようにするため
    if (A.at(i) > mean) {
      cout << A.at(i) - mean << endl;
    }
    else {
      cout << mean - A.at(i) << endl;
    }
  }
}
