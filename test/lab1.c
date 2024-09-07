int main(void)
{
  srand(time(NULL));
  int size_mass_col, size_mass_row, MAX_VAL = -40, MIN_VAL = 100;

  printf("Введите кол-во столбцов и через пробел кол-во строк > ");
  scanf("%d %d", &size_mass_col, &size_mass_row);

  int **mass = (int **)malloc(size_mass_col * sizeof(int *));
  for (int i = 0; i < size_mass_col; i++)
  {
    mass[i] = (int *)malloc(size_mass_row * sizeof(int));
  }

  for (int i = 0; i < size_mass_col; i++)
  {
    for (int j = 0; j < size_mass_row; j++)
    {
      mass[i][j] = rand() % 141 - 40;
      printf("%d ", mass[i][j]);
      if (mass[i][j] > MAX_VAL )
      {
        MAX_VAL = mass[i][j];
      }
      if (mass[i][j] < MIN_VAL)
      {
        MIN_VAL = mass[i][j];
      }
    }
    printf("\n");
  }
  printf("\n\n");

  printf("разница = %d\n", MAX_VAL - MIN_VAL);

}
