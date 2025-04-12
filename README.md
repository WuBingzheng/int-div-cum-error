# int-div-cum-error

Primitive integer division with rounding kind and cumulative error.

As we all know, integer division can lead to precision loss and errors.
What is less obvious is that in some scenarios, the dividend is split
into several parts, turning a single division into multiple divisions,
which can sometimes result in even greater errors.

For example, 60 / 3 = 20, and this division itself is error-free. However,
if we split the dividend 60 into three parts of 20 and perform the
division three times: 20 / 3 = 6.67, rounding it to 7 (using the Rounded
method as an example; other methods are similar). Adding up the three 7
gives us 21, which is 1 more than the original 20.

For such consecutive divisions, if we can record the error caused by
rounding in each division and apply it to the next division, we can
reduce or even avoid the final error.

Let's use `cum_error` to denote the cumulative error.

- The initial value is `cum_error = 0`;
- (20 + 0) / 3 = 7, cum_error = -1;
- (20 - 1) / 3 = 6, cum_error = 1;
- (20 + 1) / 3 = 7, cum_error = 0.

The final result is 7 + 6 + 7 = 20, which is equal to the result of
the single division.

This library implements this functionality.

License: MIT
