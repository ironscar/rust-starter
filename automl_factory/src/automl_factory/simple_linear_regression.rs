/*
 Method to show a demo of linear regression
 */
pub fn demo() {
    println!("Simple linear regression demo START");

    let x: [f32;5] = [1.0,2.0,3.3,4.5,5.2];
    let y: [f32;5] = [3.3,4.4,5.5,6.6,9.3];
    let alpha = 0.05;
    let iterations = 20;
    let cost_function = CostFunction::MSE;
    train(&x, &y, alpha, iterations, &cost_function);

    println!("Simple linear regression demo END");
}

/*
 Enum for the various cost functions and their implementations
 */
pub enum CostFunction {
    MSE
}
impl CostFunction {
    /*
        Method to select the cost function and return its result

        @param self - the selected cost function
        @param c - coefficients
        @param x - input data
        @param y - output data
        @returns the MSE value
     */
    fn compute_cost(&self, c: &[f32;2], x: &[f32; 5], y: &[f32; 5]) -> f32 {
        match self {
            Self::MSE => Self::compute_mse(&c, x, y)
        }
    }

    /*
        Method to get the Mean Squared Error (MSE)

        @param c - coefficients
        @param x - input data
        @param y - output data
        @returns the MSE value
    */
    fn compute_mse(c: &[f32;2], x: &[f32; 5], y: &[f32; 5]) -> f32 {
        let mut sum = 0f32;
        for i in 0..y.len() {
            let diff = predicted_val_diff(c, &[x[i]], y[i]);
            sum += diff * diff;
        }
        sum/y.len() as f32
    }
}

/*
 Method to train the simple linear regression model

 @param x - input data
 @param y - output data
 @param alpha - learning rate
 @param iterations - number of iterations to train for
 @param cost_function - the selected cost_function
 */
pub fn train(
    x: &[f32; 5],
    y: &[f32; 5],
    alpha: f32,
    iterations: i32,
    cost_function: &CostFunction
) -> [f32;2] {
    let mut cost = 0f32;
    let mut coeffs = [0f32, 0f32];

    for i in 0..iterations {
        cost = cost_function.compute_cost(&coeffs, x, y);
        println!("Iteration = {}, cost = {}, coefficients = {:?}", i+1, cost, coeffs);

        for j in 0..coeffs.len() {
            coeffs[j] -= alpha * compute_gradient(&coeffs, x, y, j as u8);
        }
    }
    println!("Final cost = {}, final coefficients = {:?}", cost, coeffs);
    coeffs
}

/*
 Method to get the gradients for coefficients

 @param m - slope
 @param b - intercept
 @param x - input data
 @param y - output data
 @param coeff_index - index of the coefficient (0 if intercept, 1 if slope)
 @returns gradient - the gradient for corresponding coefficient
 */
fn compute_gradient(
    c: &[f32;2],
    x: &[f32; 5],
    y: &[f32; 5],
    coeff_index: u8
) -> f32 {
    let mut sum = 0f32;
    for i in 0..y.len() {
        let diff = predicted_val_diff(c, &[x[i]], y[i]);
        sum += (if coeff_index != 0 {x[(coeff_index as usize) - 1]} else {1f32}) * diff;
    }
    (-2f32/(y.len() as f32)) * sum
}

/*
 Method to calculate the difference of the predicted value from actual value

 @param c - coefficients
 @param xns - list of input values
 @param yi - output value
 @returns val - the difference between predicted and actual values
 */
fn predicted_val_diff(c: &[f32;2], xns: &[f32;1], yi: f32) -> f32 {
    let mut sum = 0f32;
    for i in 1..c.len() {
        sum += c[i] * xns[i - 1];
    }
    sum + c[0] - yi
}
