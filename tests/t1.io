//README

// Each variable is defined with ceretain flags... giving it unique features. For example, <const> makes the variable immutable;

// Some flags, such as <structure>, take in an "attribute argument", like numeric (for more attribute types feel free to check out the rest of the examples, like object, string, etc. etc.); 

// As of now, <structure> is the only required flag. Even <asg>, which assigns a value to the variable, isn't "REQUIRED", for like in variable "nil_var" below, it will default to "nil", which is a sub-type of each data type in Ion.

// Right now, only log() fn is a thing... I plan on adding many more however in the future; Ion is strictly, a semicolon based language.

//Example...
| var <asg> <structure: numeric> <const> 1;  

| var_two <asg> <structure: numeric> var + 2;

var_two <asg> var_two + 1;

| nil_var <structure: numeric>; 

log( var_two + nil_var );


