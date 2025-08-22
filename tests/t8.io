| dummy <asg> <structure: array> arr [ numeric ; nil ; 0 ;];
| arr_struct <asg> <structure: array> arr [ complex ; dummy ; 1 ;];
| my_arr <asg> <structure: complex> <complex: arr_struct> [ [] ];
my_arr; 
