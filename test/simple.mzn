
var int: x;

array[int] of var int: he;

set of var int: y;

array[int] of record(var int: a): s;

constraint let { var int: x } in x < 2;
