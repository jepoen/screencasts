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
set arrow 2 from 2.5,0 to 2.5,-3.75
replot
pause -1 'Cont'
unset arrow 1
plot [0:10] x**2-10, [2.5:5] 0
pause -1 'Cont'
set arrow 4 from 3.75,0 to 3.75,4.06
replot
pause -1 'Cont'
unset arrow 5
plot [0:10] x**2-10, [2.5:3.75] 0
pause -1 'Cont'
set arrow 3 from 3.125,0 to 3.125,-0.23
replot
pause -1 'Cont'
unset arrow 2
plot [0:10] x**2-10, [3.125:3.75] 0
pause -1 'Close'
