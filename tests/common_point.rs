use common::point::Point;

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4, SQRT_2};

#[test]
fn point() {
    // New
    let p = Point::new(0.0, 0.0);
    print!("Testing Point::new()");
    assert_eq!(p, Point { x: 0.0, y: 0.0 });
    assert_ne!(p, Point { x: -1.0, y: 1.0 });
    println!(" ... ok");

    // is_valid
    let n = Point::new(std::f64::NAN, std::f64::NAN);
    let nn = Point::new(std::f64::NAN, 0.0);
    print!("Testing Point::is_valid()");
    assert_eq!(p.is_valid(), true);
    assert_eq!(n.is_valid(), false);
    assert_eq!(nn.is_valid(), false);
    println!(" ... ok");

    // module
    let p = Point::new(1.0, 1.0);
    let r = Point::new(2.0, 0.0);
    print!("Testing Point::module()");
    assert!(f64::abs(p.module() - SQRT_2) < 1e-10);
    assert!(f64::abs(r.module() - 2.0) < 1e-10);
    println!(" ... ok");

    // phase
    let p = Point::new(1.0, 1.0);
    let r = Point::new(2.0, 0.0);
    let q = Point::new(2.0, -2.0);
    print!("Testing Point::phase()");
    assert!(f64::abs(p.phase() - FRAC_PI_4) < 1e-6);
    assert!(f64::abs(r.phase() - 0.0) < 1e-6);
    assert!(f64::abs(q.phase() + FRAC_PI_4) < 1e-6);
    println!(" ... ok");

    //distance
    let z = Point::zero();
    let p = Point::new(1.0, 0.0);
    let q = Point::new(1.0, 1.0);
    print!("Testing Point::distance() and distance_to()");
    assert_eq!(z.distance_to(&p), 1.0);
    assert_eq!(Point::distance(&z, &p), 1.0);
    assert!(f64::abs(Point::distance(&z, &q) - SQRT_2) < 1e-10);
    println!(" ... ok");

    //versor
    print!("Testing Point::to_versor()");
    assert_eq!(z.to_versor(), None);
    assert_eq!(p, p.to_versor().unwrap());
    let q_ver = q.to_versor().unwrap();
    assert!(f64::abs(q_ver.x - FRAC_1_SQRT_2) < 1e-10);
    assert!(f64::abs(q_ver.y - FRAC_1_SQRT_2) < 1e-10);
    println!(" ... ok");

    //rotate_by
    let mut p = Point::new(1.0, 0.0);
    print!("Testing Point::rotate_by()");
    p.rotate_by(FRAC_PI_2);
    assert!(f64::abs(p.x - 0.0) < 1e-10);
    assert!(f64::abs(p.y - 1.0) < 1e-10);
    p.rotate_by(-FRAC_PI_4);
    assert!(f64::abs(p.x - FRAC_1_SQRT_2) < 1e-10);
    assert!(f64::abs(p.y - FRAC_1_SQRT_2) < 1e-10);
    println!(" ... ok");
}
