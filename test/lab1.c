#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
/*
struct student
{
  void famil[20];
  void name[20], facult[20];
  int Nomzach;
};
*/

void search_by_fname(const char *fname, struct student *stud, int COLVOSTUD);
void search_by_sname(const char *sname, struct student *stud, int COLVOSTUD);
void search_by_facultet(const char *facultet, struct student *stud, int COLVOSTUD);
void search_by_number(int number, struct student *stud, int COLVOSTUD);

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

  /*
   * mullti
   * line
   * comment
   */
  int COLVOSTUD;
  struct student stud[100]; // Объявление массива студентов
  for (int i = 0; ; i++) {
    printf("Введите имя студента\n> "); scanf ("%20s",stud[i].name);
    if (strcmp(stud[i].name, "*") == 0) {
      COLVOSTUD = i;
      break;
    }
    printf("Введите фамилию студента\n> ");
    scanf ("%20s",stud[i].famil);
    printf("Введите название факультета студента %s %s\n> ",stud[i].famil,stud[i].name);
    scanf ("%20s",stud[i].facult);
    printf("Введите номер зачётной книжки студента %s %s\n> ",stud[i].famil,stud[i].name);
    scanf ("%d",&stud[i].Nomzach);
    printf("Cтудент %s %s обучается на факультете %s, номер зачётной книжки %d\n> ",stud[i].famil,stud[i].name, stud[i].facult,stud[i].Nomzach);
  }

  printf("по какому параметру ищем?\n1 - имя\n2 - фамилия\n 3 - название факультета\n 4 - номер зачётной книжки\n> ");
  int search;
  scanf("%d", &search);

  // Освобождение памяти
  for (int i = 0; i < size_mass_col; i++)
  {
    free(mass[i]);
  }
  free(mass);

  return 0;
}


void search_by_fname (const char *fname, struct student *stud, int COLVOSTUD)
{
    for (int i = 0; i < COLVOSTUD; i++)
    {
        if (strcmp(stud[i].famil, fname) == 0)
        {
            printf("Cтудент %s %s обучается на факультете %s, номер зачётной книжки %d \n",stud[i].famil,stud[i].name, stud[i].facult,stud[i].Nomzach);
            return;
        }
    }
    printf("Студент %s не найден.\n", fname);
}

void search_by_sname(const char *sname, struct student *stud, int COLVOSTUD)
{
    for (int i = 0; i < COLVOSTUD; i++)
    {
        if (strcmp(stud[i].name, sname) == 0)
        {
            printf("Cтудент %s %s обучается на факультете %s, номер зачётной книжки %d \n",stud[i].famil,stud[i].name, stud[i].facult,stud[i].Nomzach);
            return;
        }
    }
    printf("Cтудент %s не найден.\n", sname);
}

void search_by_facultet(const char *facultet, struct student *stud, int COLVOSTUD)
{
    for (int i = 0; i < COLVOSTUD; i++)
    {
        if (strcmp(stud[i].facult, facultet) == 0)
        {
            printf("Cтудент %s %s обучается на факультете %s, номер зачётной книжки %d \n",stud[i].famil,stud[i].name, stud[i].facult,stud[i].Nomzach);
            return;
        }
    }
    printf("Студент не найден на факультете %s.\n", facultet);
}

void search_by_number(int number, struct student *stud, int COLVOSTUD)
{
    for (int i = 0; i < COLVOSTUD; i++)
    {
        if (stud[i].Nomzach == number)
        {
            printf("Cтудент %s %s обучается на факультете %s, номер зачётной книжки %d \n",stud[i].famil,stud[i].name, stud[i].facult,stud[i].Nomzach);
            return;
        }
    }
    printf("Нет книжки с номером %d.\n", number);
}
