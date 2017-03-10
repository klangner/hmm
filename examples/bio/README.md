# DNA classification example

In this example we will build classifier which will split DNA into coding and non-coding regions.
The coding regions can be recognized because it has more GC content.

The probability for coding segment:
 
  * A: 0.18
  * C: 0.32
  * G: 0.32
  * T: 0.18
  
  
The probability for non-coding  
 
  * A: 0.25
  * C: 0.25
  * G: 0.25
  * T: 0.25


The probability of state changing:

  * Stay in the same state: 0.98
  * change state: 0.02
  
Decode sequence ATGCGA  