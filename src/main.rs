extern crate uom;

use uom::si::f32::*;
use uom::si::length::{kilometer,foot};
use uom::si::time::second;

// import units
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::velocity::meter_per_second;
use uom::si::length::meter;
use uom::si::dynamic_viscosity::pascal_second;

fn main() {
    println!("Hello, world!");
    uom_demo();
    reynolds_demo();
    unit_conversion_demo();

}

fn reynolds_demo(){
    println!("reynolds demo!");

    let rho = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
    let u = Velocity::new::<meter_per_second>(0.005);
    let d = Length::new::<meter>(2.88e-2);
    let mu = DynamicViscosity::new::<pascal_second>(0.001);

    let reynolds_number = calc_reynolds(rho, u, d, mu);

    println!("Reynolds Number = {} ", reynolds_number);

}

fn calc_reynolds(rho: MassDensity,
    velocity: Velocity, 
    hydraulic_diameter: Length,
    mu: DynamicViscosity) -> f64 {

    let reynolds_number = 
        rho * 
        velocity * 
        hydraulic_diameter / 
        mu;

    return convert_dimensionless_number_into_double(reynolds_number);

}

fn convert_dimensionless_number_into_double(
    dimensionless_number: Ratio) -> f64 {
    
    return dimensionless_number.value.into();

}

fn unit_conversion_demo() -> f64{

    println!("\n unit conversion demo");

    let d = Length::new::<kilometer>(1.0);
    let d_converted = d.get::<kilometer>();

    println!("{:?} = {:?}", d, d_converted);

    return convert_dimensionless_number_into_double(d/d);

}


// uom demo code block or fns

fn uom_demo(){
    let length = Length::new::<kilometer>(5.0);
    let time = Time::new::<second>(15.0);
    let velocity/*: Velocity*/ = length / time;
    let _acceleration = calc_acceleration(velocity, time);
    //let error = length + time; // error[E0308]: mismatched types
    println!("{:?}", _acceleration);
}

fn calc_acceleration(velocity: Velocity, time: Time) -> Acceleration {
    velocity / time
}
