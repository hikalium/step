int a[N][N];
int b[N][N];
int c[N][N];
int i, j, k;
int main() {
  for (j = 0; j < N; j++)
    for (k = 0; k < N; k++)
      for (i = 0; i < N; i++)
        c[i][j] += a[i][k] * b[k][j];
}
