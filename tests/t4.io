| obj_e <asg> <structure: object> //Error msgs help!
obj { 
    x: numeric; 
};
| dummy <asg> <structure: array> arr [ complex ; obj_e ; 1 ;];
| my_arr <asg> <structure: complex> <complex: dummy> [ 2 ];
log(my_arr); 

