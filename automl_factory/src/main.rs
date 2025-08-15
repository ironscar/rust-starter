mod automl_factory;

pub fn main() {
    let current_automl_index = 1;
    match current_automl_index {
        1 => {
            automl_factory::simple_linear_regression::demo();
        },
        _ => {
            println!("Unknown automl implementation");
        }
    };
}
