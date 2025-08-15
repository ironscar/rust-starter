# Simple Linear Regression

## Introduction

- Referring to https://www.geeksforgeeks.org/machine-learning/ml-linear-regression/
- This is basically `y = mx + b` where m is slope and b is base
- Finds the best value of m and b that matches the observed data
- Usually this is done with gradient descent

## Gradient Descent

- Start with random initial values of m and b
- Calculate cost MSE = `sum((yi - y)^2)/n` where (xi,yi) is each datapoint, y is predicted value and n is data size
- for m, `gradient_m = -(2/n)*sum(xi*(yi - y))`
- for b, `gradient_b = -(2/n)*sum(yi - y)`
- the gradients are basically the partial derivative of the cost function based on m and b respectively
- update ma and b as `m -= alpha*gradient_m` and `b -= alpha*gradient_b` where `alpha` is learning rate
  - alpha acts as a hyperparameter
- keep repeating this until error stops reducing significantly
  - need to figure out what is a good time to stop iterating but for now we will take iterations as a parameter
- for more details, refer https://www.geeksforgeeks.org/machine-learning/gradient-descent-in-linear-regression/

## Normal Equation

- For the special case of linear regressions, we can use the `Normal equation`
- `C = inverse(transpose(X)*X)*(transpose(X)*Y)`
- X is matrix of input features
- Y is matrix of output results
- C is matrix of coefficients
- for small datasets, this can help getting all coefficients in one shot without iterative trial-and-error
- for large datasets, the inverse can become computationally expensive
- this selection can be part of the AutoML selector based on data size
  - need to find at what data size things tend to be more expensive for this
- for more details, refer https://www.geeksforgeeks.org/machine-learning/ml-normal-equation-in-linear-regression/
