use bevy::prelude::*;

pub fn cos_distance(a: &Vec<f64>, b: &Vec<f64>) -> f64{
    // dot-product(u, v) / sqrt(dot-product(u, u))*sqrt(dot-product(v,v))
    let a_sum: f64 = dot_product(a, a).sqrt();
    let b_sum: f64 = dot_product(b, b).sqrt();
    // info!("a:{:?}, b:{:?}", a_sum, b_sum);

    let dist: f64 = dot_product(a, b)/(a_sum * b_sum) ;
    return dist;
}

pub fn dot_product(a: &Vec<f64>, b: &Vec<f64>) -> f64{
    let mut sum = 0.0;
    //are assuming len are equal
    if a.len() != b.len() {
        info!("Warning! dot_product called on unequal vectors! {},{}", a.len(), b.len());
    }else {
        for i in 0..a.len() {
            sum += a[i] * b[i];
        }
    }
    //println!("{:?}", sum);
    if sum == 0.0 {return 1.0;} // bad idea but need to not have a zero to sqrt
    return sum;
}

pub fn sum(a: &mut Vec<f64>, b: &Vec<f64>){
    if a.len() != b.len() {panic!("Warning! sum called on unequal vectors! {},{}", a.len(), b.len());}
    let mut j = 0;
    for i in a.iter_mut(){
        *i += b[j];
        j +=1;
    }
}
