# Math utilities module

fn | math_add (int | a, int | b) > a + b

fn | math_multiply (int | a, int | b) > a * b

fn | math_square (int | n) > n * n

fn int | math_factorial (int | n) > {
  if n <= 1 {
    1
  } else {
    n * math_factorial(n - 1)
  }
}

int | PI_APPROX -> 3
int | E_APPROX -> 2
