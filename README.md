# Asive_SIbeko-LInear_Rust

**Linear Regression Model in Rust**
**Project Overview**
This project implements a basic linear regression model in Rust.
The model is designed to learn a simple linear relationship from synthetic data, using gradient descent for training. 
While the training and prediction parts work correctly, there are some dependency warnings related to the rand crate (which uses deprecated functions).

**What is Working**
Model Training:
The model trains correctly using gradient descent. Loss values decrease over epochs, showing the model learns from the data.

Predictions:
After training, the model prints predictions for given x values.

Model Predictions:
x: 0, predicted y: 1.062
x: 1, predicted y: 3.398
x: 2, predicted y: 5.734
x: 3, predicted y: 8.070
x: 4, predicted y: 10.406
This confirms that the core functionality of the model works as expected.


**What is Not Working**-
**Dependency Issues:**
The import for certain parts of the rand crate (like rand::distributions::{Distribution, Uniform}) is causing errors.
The function rand::thread_rng() is marked as deprecated in our version, and the new recommended methods are not fully integrated in our code.
As a result, there are warnings about deprecated functions and unresolved imports.
**Borrowing Errors:**
There was a conflict when calling the chart plotting functions. 
Rust does not allow borrowing the same variable (chart) twice at the same time, which caused a mutable borrowing error.

**What I Learned from This Experience**
**Working Parts:**
The model trains and makes predictions correctly, which shows that the basic logic of the linear regression implementation is solid.
**Fixing Errors Takes Time:**
I learned that resolving errors in Rust can be challenging. 
It requires reading error messages carefully, trying multiple solutions, and checking documentation. 
Even if some parts are working, dependency issues can prevent the entire program from running smoothly.

**Managing Dependencies:**
Getting the right versions of libraries and knowing how to import and use them is very important. 
I learned that small changes in dependency versions can cause warnings or errors, and that keeping everything updated and consistent is key.
