//f(x) = sinx integrated over -pi to pi
fn f(x: f64) -> f64 {        
        return x.cos();
    }

fn main() {

    //integration limits
   // let a = -(std::f64::consts::PI);
   // let b = std::f64::consts::PI;
  // let a = -1.0;
   //let b = 1.0;
   let a = 0.0;
   let b = 2.0*(std::f64::consts::PI);

    //number of steps
    let n=1000;


    let result_trapezoid = trapezoidal(a, b, n);
    let result_simpsons = simpsons(a, b, n);
    let result_simpsons38 = simpsons38(a, b, n);
    let result_booles = booles(a, b, n);
    println!("cosx: trapezoid rule: {}", result_trapezoid);
    println!("cosx: simpsons rule: {}", result_simpsons);
    println!("cosx: simpsons 3/8 rule: {}", result_simpsons38);
    println!("cosx: booles rule: {}", result_booles);


}

fn trapezoidal(a: f64,b: f64, n: i32) -> f64{
    //"height" of each trapezoidal step
    let h = (b-a)/n as f64;
    
    let mut result_trapezoidal = 0.0;
    for i in 0..=n {
        let x_i = i as f64 * h + a;
        let y_i = f(x_i);

        if i != 0 && i != n {
            result_trapezoidal = result_trapezoidal + y_i;
        } 
        else {
            result_trapezoidal = result_trapezoidal + y_i/2.0;
        }
    }

   result_trapezoidal = result_trapezoidal*h;
    return result_trapezoidal;
   
}

fn simpsons(a: f64, b: f64, n: i32) -> f64{
    //step size
    let h = (b-a)/n as f64;
    
    let mut result_simpsons = f(a) + f(b);

    for i in 1..=n-1 {
        let x_i = i as f64 * h + a;
        
        
        let weight =
            if i%2 == 0 {
                2.0
            }
            else {
                4.0
            };

        result_simpsons = result_simpsons + weight*f(x_i);
    
    }

    result_simpsons = (result_simpsons*h)/3.0;
    return result_simpsons;

}

fn simpsons38(a: f64, b: f64, n: i32) -> f64 {
    //step size
    let h = (b-a)/n as f64;
    
    let mut result_simpsons38 = f(a) + f(b);

    for i in 1..=n-1 {
        let x_i1 = i as f64 * h + a;
        let coeff =       //
            if i % 3 == 0 { 
                2.0 
            } 
            else { 
                3.0 
            };   //
        result_simpsons38 = result_simpsons38 + coeff*f(x_i1);
    }
    
    /*for j in 2..n-2 {
        let x_i2 = j as f64 *h + a;
        sum = sum + 2.0*f(x_i2);
    }*/

    result_simpsons38 = (result_simpsons38*h*3.0)/8.0;
    return result_simpsons38;
}

fn booles(a: f64, b: f64, n: i32) -> f64 {
    //step size
    let h = (b-a)/n as f64;
    
    let mut result_booles = 7.0*f(a) + 7.0*f(b);

    for i in 1..=n-1 {
        let x_i = i as f64 * h + a;
        let coeff =       
            if i % 4 == 0 { 
                7.0 
            } 
            else if (i % 2 == 0) && (i % 4 !=0) { 
                32.0
            }
            else {
                12.0
            };   
        result_booles = result_booles + coeff*f(x_i);
    }
    

    result_booles = (result_booles*h*2.0)/45.0;
    return result_booles;
}