$obj_e <asg> <structure: object> 
{ 
    x: numeric; 
};
$obj_main <asg> <structure: object> 
{ 
    x: numeric; 
    y: obj_e;
};
$obj <asg> <structure: complex> <complex: obj_main> 
obj_main{
    x: 2; 
    y: obj_e{x: 6;};
};

obj
