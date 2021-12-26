## Fast Totient Sieve

1. Initialize an array from 2:n with the values 2:n. Start from m = 2.
2. If you are visiting position  m  and it's value is still  m , this is a prime number, so perform the next step (otherwise skip)
3. Visit  2m,3m,4m,...  and multiply by  (1−1/m) 
4.  m=m+1  and go back to step 2

Now, this is pretty simple but it requires a division at each iteration of step 3. We can instead start from 1 and only have to do multiplication:

1. Initialize the array from 2:n with the value 1, and start at m = 2.
2. If the value at m is still 1, m is prime.
3. Calculate the largest  mk≤n  and set  t[i]=mk−i−1(m−1)  via repeated multiplication. Visit  2mk,3mk,...  and multiply those entries by  t[k] .
4. Do the same for  mk−1  but instead skip the multiples of  m  while iterating. I.e., if m=2 then visit  mk−1,3mk−1,5mk−1,...  and multiply each by  t[k−1] . Repeat for all powers of  m . including  m1 .
5. Increment m and go back to step 2

Step 4 can be done as a double-loop to avoid a division check. (We need to not visit  xmk  and then visit it again as a multiple of  m  or  m2 .)