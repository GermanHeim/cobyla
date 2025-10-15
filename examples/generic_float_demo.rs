// This example demonstrates the use of the `cobyla` crate with both `f32` and `f64` types.
// It defines a simple quadratic objective function and minimizes it using the COBYLA algorithm.

use cobyla::*;

fn obj_f64(x: &[f64], _: &mut ()) -> f64 {
    (x[0] - 2.0).powi(2) + (x[1] - 3.0).powi(2)
}

fn obj_f32(x: &[f32], _: &mut ()) -> f32 {
    (x[0] - 2.0).powi(2) + (x[1] - 3.0).powi(2)
}

fn main() {
    // Test with f64
    {
        println!("Using f64");
        let xinit_f64 = vec![1.0_f64, 1.0_f64];
        let bounds_f64 = vec![(-10.0, 10.0), (-10.0, 10.0)];
        
        let cons_f64: Vec<&dyn Fn(&[f64], &mut ()) -> f64> = vec![];

        let result_f64 = minimize(
            obj_f64,
            &xinit_f64,
            &bounds_f64,
            &cons_f64,
            (),
            1000,
            RhoBeg::All(0.5),
            Some(StopTols::default()),
        );

        match &result_f64 {
            Ok((status, x, fopt)) => {
                println!("Status: {:?}", status);
                println!("Optimal x: [{:.6}, {:.6}]", x[0], x[1]);
                println!("Expected: [2.0, 3.0]");
                println!("Optimal f(x): {:.10}", fopt);
            }
            Err(e) => println!("Failed: {:?}", e),
        }
        println!();
    }

    // Test with f32
    {
        println!("Using f32");
        let xinit_f32 = vec![1.0_f32, 1.0_f32];
        let bounds_f32 = vec![(-10.0, 10.0), (-10.0, 10.0)];
        
        let cons_f32: Vec<&dyn Fn(&[f32], &mut ()) -> f32> = vec![];

        // Use reasonable tolerances for f32 precision
        // Without this, the default tolerances are too tight for f32
        // and we end with a failure do to RoundoffLimited
        let stop_tol_f32 = StopTols {
            ftol_rel: 1e-6,
            xtol_rel: 1e-6,
            ..StopTols::default()
        };

        let result_f32 = minimize(
            obj_f32,
            &xinit_f32,
            &bounds_f32,
            &cons_f32,
            (),
            1000,
            RhoBeg::All(0.5),
            Some(stop_tol_f32),
        );

        match &result_f32 {
            Ok((status, x, fopt)) => {
                println!("Status: {:?}", status);
                println!("Optimal x: [{:.6}, {:.6}]", x[0], x[1]);
                println!("Expected: [2.0, 3.0]");
                println!("Optimal f(x): {:.10}", fopt);
            }
            Err(e) => println!("Failed: {:?}", e),
        }
        println!();
    }
}
