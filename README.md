# const-sort
This Library provides a Bitonic Sorting Network which makes a best-effort to
sort a slice of elements in constant time (with respect to slice length).
Obviously, different length slices will take different amounts of time and code
caching may make different iterations of the algorithm take different amounts of
time. The goal of this crate is to provide a sorting algorithm which takes the
same amount of time irrespective of the passed in slice's values.

The constant time-ness has not been rigorously tested as this library is under
development. Use at your own peril. You must pass in a constant-time comparator
function of your own.

The sort provided is not intended to be high performance.
