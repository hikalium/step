int a[N][N];
int b[N][N];
int c[N][N];
int i, j, k;
int main() {
  for (i = 0; i < N; i++)
    for (k = 0; k < N; k++)
      for (j = 0; j < N; j++)
        c[i][j] += a[i][k] * b[k][j];
}
