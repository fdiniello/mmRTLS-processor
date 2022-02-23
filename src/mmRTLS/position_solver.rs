use itertools::Itertools;
use std::collections::HashMap;

use common::{
    influxdb_models::{BeaconMeasure, KnownPosition},
    Antenna, Point, MAC,
};
struct KnownDistance {
    point: Point,
    dist: f64,
}

pub async fn solve_for(device_id: MAC) -> Result<Point, ()> {
    let antennas = anntennas_hashmap();

    let measure = BeaconMeasure::get_for(device_id.as_str()).await.unwrap();

    let known_distance = measure
        .iter()
        .filter_map(|m| {
            if let Some(a) = antennas.get(&m.beacon_id) {
                let kd = KnownDistance {
                    point: a.coord,
                    dist: a.get_distance_with_W(m.rssi),
                };
                Some(kd)
            } else {
                None
            }
        })
        .collect::<Vec<KnownDistance>>();

    let mut posible_positions = known_distance
        .iter()
        .permutations(3)
        .filter_map(|per| trilat(per[0], per[1], per[2]))
        .collect::<Vec<KnownDistance>>();

    print!("Old len(): {} \t", posible_positions.len());

    if let Some(last_position) = KnownPosition::get_last_for(device_id.as_str(), 2)
        .await
        .unwrap()
    {
        let last_position = Point::new(last_position.x, last_position.y);
        posible_positions.retain(|p| last_position.distance_to(&p.point) < 3.0);
    }
    println!("New len(): {}", posible_positions.len());

    let mut pos = Point::new(0.0, 0.0);
    let mut divisor = 0.0;
    for p in posible_positions.iter() {
        pos.x += p.point.x / p.dist;
        pos.y += p.point.y / p.dist;
        divisor += 1.0 / p.dist;
    }

    pos /= divisor;

    // println!("Pos: {}", pos);
    let _r = KnownPosition::new(pos).write_for(device_id.as_str()).await;

    Ok(pos)
}

fn trilat(a: &KnownDistance, b: &KnownDistance, c: &KnownDistance) -> Option<KnownDistance> {
    #![allow(non_snake_case)]

    let points = vec![a.point, b.point, c.point];
    for &p in points.iter() {
        if !p.is_valid() {
            return None;
        }
    }

    // We have two triangles that share a side,
    // Da and Db are both a hypotenuse,
    // h is the shared side
    // D is the lineal sum of both coaxial sides.
    //          P
    //         /|\
    //        / | \
    //     Da/  |h \Db
    //      /   |   \
    //     / d1 | d2 \
    //    *-----------*
    //    A           B => D = BA

    let D = (b.point - a.point).module();

    let d1 = (D.powi(2) + a.dist.powi(2) - b.dist.powi(2)) / (2.0 * D);
    let h = f64::sqrt(a.dist.powi(2) - d1.powi(2));
    if h.is_nan() {
        return None;
    }

    // With points A and B, we can find the Position P, but we the fact is that there are
    // two posible solutions, we build a rhombus with both posible P:
    let D_ver = (b.point - a.point).to_versor().unwrap();

    let mut upper = D_ver * a.dist;
    let mut downer = D_ver * a.dist;

    // we need to rotate that direction by alpha and -alpha
    let alpha = f64::tan(h / d1);
    upper.rotate_by(alpha);
    downer.rotate_by(-alpha);

    // Now we have two vectors with |Da| that point from A where the two posible positions are
    let P = [a.point + upper, a.point + downer];

    //Now we need to see which P[0] or P[1] is at distance Dc from pointC.
    //But since all numbers we got (Da,Db and Dc) cointain a lot of error and noise
    // we know that they won't be the same number so we need to pick the point that makes the distance to pointC the closest to Dc

    let dist_to_C = [P[0].distance_to(&c.point), P[1].distance_to(&c.point)];
    let error = [
        f64::abs(dist_to_C[0] - c.dist),
        f64::abs(dist_to_C[1] - c.dist),
    ];

    if error[0] < error[1] {
        Some(KnownDistance {
            point: P[0],
            dist: error[0],
        })
    } else {
        Some(KnownDistance {
            point: P[1],
            dist: error[1],
        })
    }
}

fn anntennas_hashmap() -> HashMap<MAC, Antenna> {
    let data = vec![
        Antenna::new("e6:ad:0b:2e:d7:11", 30.0, Point::new(15.0, 15.0)),
        Antenna::new("c2:b5:f5:cc:e6:88", 30.0, Point::new(15.0, -15.0)),
        Antenna::new("e6:2e:e6:88:f5:cc", 30.0, Point::new(-15.0, 15.0)),
        Antenna::new("c2:ad:0b:b5:11:d7", 30.0, Point::new(-15.0, -15.0)),
    ];
    let mut map: HashMap<MAC, Antenna> = HashMap::new();
    for a in data.iter() {
        map.insert(a.id.clone(), a.clone());
    }
    map.into()
}

#[test]
fn test_trilat() {
    let a = KnownDistance {
        dist: 6.3,
        point: Point::new(0.0, 0.0),
    };
    let b = KnownDistance {
        dist: 3.1,
        point: Point::new(5.0, 6.5),
    };
    let c = KnownDistance {
        dist: 5.5,
        point: Point::new(9.0, 0.0),
    };

    let pos = trilat(&a, &b, &c).unwrap();
    let expected = Point::new(5.0, 3.5);

    assert!(f64::abs(pos.point.x - expected.x) < 0.5);
    assert!(f64::abs(pos.point.y - expected.y) < 0.5);
}
