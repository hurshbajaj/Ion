| obj_e <asg> <structure: object> 
obj { 
    x: numeric; 
};
| obj_main <asg> <structure: object> 
obj { 
    x: numeric; 
    y: obj_e;
};
| Obj <asg> <structure: complex> <complex: obj_main> 
{
    x: 2; 
    y: obj_e{x: 6;};
};

Obj.x;

| ObjA <asg> <structure: complex> <complex: anonymous> 
{
    x: 2; 
};

ObjA.x;


