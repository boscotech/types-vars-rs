//calculate the midpoint

fn midpoint() -> ((f64, f64), (f64, f64), (f64, f64)) {
    //declare a mutable f64 tuple as (0, 0)
    let mut result: (f64, f64) = (0.0, 0.0);
    //declare your first set of coordinates as a f64 tuple
    let point1: (f64, f64) = (1.0, 2.0);
    //declare your second set of coordinates as a f64 tuple
    let point2: (f64, f64) = (2.0, 3.0);
    //calculate the midpoint of the x coordinates and store it into result.0
    result.0 = (point1.0 + point2.0) / 2.0;
    //calculate the midpoint of the y coordinates and store it into result.1
    result.1 = (point1.1 + point2.1) / 2.0;
    //print the result using debug print
    println!("{:?}", result);

    //"freeze" the result here
    let result = result;
    //don't worry about this
    return (result, point1, point2);
}

fn main() {
    midpoint();
}

#[cfg(test)]
mod test;
