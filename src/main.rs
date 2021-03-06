#![feature(iter_arith)]
extern crate nn;
use nn::{NN, HaltCondition};

fn main() {
    // create examples of the XOR function
    // the network is trained on tuples of vectors where the first vector
    // is the inputs and the second vector is the expected outputs
    let xorExamples = [
        (vec![0f64, 0f64], vec![0f64]),
        (vec![0f64, 1f64], vec![1f64]),
        (vec![1f64, 0f64], vec![1f64]),
        (vec![1f64, 1f64], vec![0f64]),
    ];

    let xnorExamples = [
        (vec![0f64, 0f64], vec![1f64]),
        (vec![0f64, 1f64], vec![0f64]),
        (vec![1f64, 0f64], vec![0f64]),
        (vec![1f64, 1f64], vec![1f64]),
    ];

    let andExamples = [
        (vec![0f64, 0f64], vec![0f64]),
        (vec![0f64, 1f64], vec![0f64]),
        (vec![1f64, 0f64], vec![0f64]),
        (vec![1f64, 1f64], vec![1f64]),
    ];

    // create a new neural network by passing a pointer to an array
    // that specifies the number of layers and the number of nodes in each layer
    // in this case we have an input layer with 2 nodes, one hidden layer
    // with 3 nodes and the output layer has 1 node
    let mut net = NN::new(&[2, 3, 1]);

    // train the network on the examples of the XOR function
    // all methods seen here are optional except go() which must be called to begin training
    // see the documentation for the Trainer struct for more info on what each method does
    println!("Training XOR");
    net.train(&xorExamples)
        .halt_condition( HaltCondition::Epochs(10000) )
        .log_interval( Some(100) )
        .momentum( 0.1 )
        .rate( 0.3 )
        .go();

    println!("Evaluating XOR performance");
    // evaluate the network to see if it learned the XOR function
    for &(ref inputs, ref outputs) in xorExamples.iter() {
        let results = net.run(inputs);
        let (result, key) = (results[0].round(), outputs[0]);
        assert!(result == key);
    }
    println!("Passed XOR assertions");

    println!("Training XNOR");
    net.train(&xnorExamples)
        .halt_condition( HaltCondition::Epochs(10000) )
        .log_interval( Some(100) )
        .momentum( 0.1 )
        .rate( 0.3 )
        .go();

    println!("Evaluating XNOR performance");
    // evaluate the network to see if it learned the XOR function
    for &(ref inputs, ref outputs) in xnorExamples.iter() {
        let results = net.run(inputs);
        let (result, key) = (results[0].round(), outputs[0]);
        assert!(result == key);
    }
    println!("Passed XNOR assertions");


    println!("Training AND");
    net.train(&andExamples)
        .halt_condition( HaltCondition::Epochs(10000) )
        .log_interval( Some(100) )
        .momentum( 0.1 )
        .rate( 0.3 )
        .go();

    println!("Evaluating AND performance");
    // evaluate the network to see if it learned the XOR function
    for &(ref inputs, ref outputs) in andExamples.iter() {
        let results = net.run(inputs);
        let (result, key) = (results[0].round(), outputs[0]);
        assert!(result == key);
    }
    println!("Passed AND assertions")
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_sum() {
        // Floats Array
        let expectedFloatSum = 11.0;
        let inputFloatArr: [f64; 4] = [1.0,2.0,2.0,6.0];
        let actualFloatSum = inputFloatArr.iter().sum();
        assert!(expectedFloatSum == actualFloatSum);

        // Integer Array
        let expectedIntSum = 11;
        let inputIntArr: [i32; 4] = [1,2,2,6];
        let actualIntSum = inputIntArr.iter().sum();
        assert!(expectedIntSum == actualIntSum);

    }
}
