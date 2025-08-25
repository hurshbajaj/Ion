| obj_e <asg> <structure: object> //The object attribute argument for the flag: structure, provides a struct... Likewise with array;
obj { 
    x: numeric; 
};
| dummy <asg> <structure: array> arr [ complex ; obj_e ; 1 ;];
| my_arr <asg> <structure: complex> <complex: dummy> [ 2 ];
log(my_arr); 

