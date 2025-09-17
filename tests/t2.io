| obj_e <asg> <structure: object> //The object attribute argument for the flag: structure, provides a struct... Likewise with array; NOTE: for structs, the body must be preceeded by its respective keyword, for eg. arr, obj, etc.
obj { 
    x: numeric; 
};
| obj_main <asg> <structure: object> 
obj { 
    x: numeric; 
    y: obj_e;
};
| Obj <asg> <structure: complex> <complex: obj_main> //Complex then further utilizes these structs as such.
{
    x: 2; 
    y: {x:1;};
};

log(Obj.y);

| ObjA <asg> <structure: complex> <complex: anonymous> //More on attribute-argument:anonymous later on...
{
    x: 2; 
};

log(ObjA.x);

