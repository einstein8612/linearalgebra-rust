# Mnist

### About
This is an example on a simple deep learning algorithm as a proof of concept. It uses my [linear algebra library](https://github.com/einstein8612/linearalgebra-rust).

### How it works
It's a simple 2 layer neural network using a rectified linear unit activation function and the normalized exponential function as the respective "delinearisation" functions.

The first layer turns the 28*28 vector into a 10 long vector. The second layer produces the final result.

### How to start
After cloning the repo you can:

*Note: Make sure you have the newest version of Rust nightly installed.*

1. Run the ./run.sh script which will automatically download the training data and run the application
2. Do it manually
    1. Download the training data yourself. It's widely available online.
    2. Call it mnist_train.csv and put it in the root folder.
    3. Run `cargo run --release`