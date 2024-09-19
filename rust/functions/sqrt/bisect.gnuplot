set title 'Bisection algorithm for x = √a or zero of x²-a (a = 10)'
set grid
set arrow 1 from 0,0 to 0,-10
set arrow 10 from 10,0 to 10,90
plot [0:10] x**2-10, [0:10] 0
pause -1 'Cont'
set arrow 5 from 5,0 to 5,15
replot
pause -1 'Cont'
unset arrow 10
plot [0:10] x**2-10, [0:5] 0
pause -1 'Cont'
set arrow 2 from 2,0 to 2,-6
replot
pause -1 'Cont'
unset arrow 1
plot [0:10] x**2-10, [2:5] 0
pause -1 'Cont'
set arrow from 3,0 to 3,-1
replot
pause -1 'Cont'
unset arrow 2
plot [0:10] x**2-10, [3:5] 0
pause -1 'Cont'
set arrow from 4,0 to 4,6
replot
pause -1 'Cont'
unset arrow 5
plot [0:10] x**2-10, [3:4] 0
pause -1 'Close'
