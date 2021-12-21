use std::{collections::{HashMap, HashSet}, hash::Hash, ops::Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
enum O {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11,
    _12,
    _13,
    _14,
    _15,
    _16,
    _17,
    _18,
    _19,
    _20,
    _21,
    _22,
    _23,
    _24,
}

fn apply_orientation_to_point(o: &O, v: &Vec3) -> Vec3 {
    let x = v.0;
    let y = v.1;
    let z = v.2;
    match o {
        O::_1 => (x, y, z),
        O::_2 => (-z, y, x),
        O::_3 => (y, -x, z),
        O::_4 => (y, z, x),
        O::_5 => (-z, -x, y),
        O::_6 => (-x, -y, z),
        O::_7 => (-x, z, y),
        O::_8 => (-z, -y, -x),
        O::_9 => (-y, x, z),
        O::_10 => (-y, z, -x),
        O::_11 => (-z, x, -y),
        O::_12 => (x, z, -y),
        O::_13 => (y, x, -z),
        O::_14 => (y, -z, -x),
        O::_15 => (z, x, y),
        O::_16 => (x, -y, -z),
        O::_17 => (x, -z, y),
        O::_18 => (z, -y, x),
        O::_19 => (-y, -x, -z),
        O::_20 => (-y, -z, x),
        O::_21 => (z, -x, -y),
        O::_22 => (-x, y, -z),
        O::_23 => (-x, -z, -y),
        O::_24 => (z, y, -x),
    }
}

fn all_rotations_of_point(v: &Vec3) -> [(O, Vec3); 24] {
    let x = v.0;
    let y = v.1;
    let z = v.2;
    return [
        (O::_1, (x, y, z)),
        (O::_2, (-z, y, x)),
        (O::_3, (y, -x, z)),
        (O::_4, (y, z, x)),
        (O::_5, (-z, -x, y)),
        (O::_6, (-x, -y, z)),
        (O::_7, (-x, z, y)),
        (O::_8, (-z, -y, -x)),
        (O::_9, (-y, x, z)),
        (O::_10, (-y, z, -x)),
        (O::_11, (-z, x, -y)),
        (O::_12, (x, z, -y)),
        (O::_13, (y, x, -z)),
        (O::_14, (y, -z, -x)),
        (O::_15, (z, x, y)),
        (O::_16, (x, -y, -z)),
        (O::_17, (x, -z, y)),
        (O::_18, (z, -y, x)),
        (O::_19, (-y, -x, -z)),
        (O::_20, (-y, -z, x)),
        (O::_21, (z, -x, -y)),
        (O::_22, (-x, y, -z)),
        (O::_23, (-x, -z, -y)),
        (O::_24, (z, y, -x)),
    ];
}

type Vec3 = (i64, i64, i64);

struct Sensor {
    raw_readings: Vec<Vec3>,
    relative_beacon_coords_per_beacon_per_o: HashMap<usize, Vec<(O, Vec<Delta>)>>,
}

struct Problem {
    sensors: Vec<Sensor>,
}

#[derive(Debug)]
struct Delta {
    raw_reading_index: usize,
    offset_from_root: Vec3,
    root_index: usize
}

fn add_vec3s(a: &Vec3, b: &Vec3) -> Vec3 {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

impl Problem {
    fn from_file(path: &str) -> Result<Problem, String> {
        let res = std::fs::read_to_string(path);
        let file_str = res.map_err(|e| e.to_string())?;
        let lines: Vec<String> = file_str
            .split("\n")
            .map(|line| line.trim().to_string())
            .collect();
        let mut idx = 0;
        let mut sensors = vec![];
        loop {
            if lines[idx].contains("scanner") {
                idx += 1;
                let mut coords = vec![];
                while idx < lines.len() && lines[idx].trim().len() > 0 {
                    let line_split: Vec<&str> = lines[idx].trim().split(",").collect();
                    let x: i64 = line_split[0].parse().unwrap();
                    let y: i64 = line_split[1].parse().unwrap();
                    let z: i64 = line_split[2].parse().unwrap();
                    coords.push((x, y, z));
                    idx += 1;
                }
                if coords.len() > 0 {
                    println!("Sensor {} -> read {} beacons", sensors.len(), coords.len());
                    sensors.push(Sensor {
                        relative_beacon_coords_per_beacon_per_o: Problem::fingerprint_readings(
                            &coords,
                        ),
                        raw_readings: coords,
                    });
                    idx += 1; // advance past empty line
                }
            } else {
                idx += 1;
            }
            if idx >= lines.len() {
                break;
            }
        }
        println!("Read {} sensor readings", sensors.len());
        Ok(Problem { sensors })
    }

    fn fingerprint_readings(
        readings: &Vec<(i64, i64, i64)>,
    ) -> HashMap<usize, Vec<(O, Vec<Delta>)>> {
        let mut fingerprints = HashMap::new();
        for (root_beacon_idx, root_beacon) in readings.iter().enumerate() {
            let all_orientations_of_root = all_rotations_of_point(root_beacon);
            let mut deltas: Vec<(O, Vec<Delta>)> = vec![];
            for (o, point) in all_orientations_of_root {
                let mut relative_other_beacons: Vec<Delta> = vec![];
                for (other_beacon_idx, other_beacon) in readings
                    .iter()
                    .map(|b| apply_orientation_to_point(&o, b))
                    .enumerate()
                {
                    let offset = (
                        other_beacon.0 - point.0,
                        other_beacon.1 - point.1,
                        other_beacon.2 - point.2,
                    );
                    relative_other_beacons.push(Delta {
                        offset_from_root: offset,
                        raw_reading_index: other_beacon_idx,
                        root_index: root_beacon_idx
                    })
                }
                deltas.push((o, relative_other_beacons))
            }
            fingerprints.insert(root_beacon_idx, deltas);
        }
        fingerprints
    }
    // fn memoize_intersection_by_dist<'b>(
    //     &self,
    //     lhs: usize,
    //     rhs: usize,
    //     memo: &'b mut HashMap<(usize, usize), BeaconIntersection>,
    // ) {
    //     if !memo.contains_key(&(lhs, rhs)) {
    //         let left = &self.sensors[lhs];
    //         let right = &self.sensors[rhs];
    //         let dist_based_beacon_intersection = left.detect_beacon_intersection_by_dist(right);
    //         memo.insert((lhs, rhs), dist_based_beacon_intersection);
    //     }
    // }

    fn detect_nearby_sensors<'b>(
        &self,
        cur_sensor_idx: usize,
        cur_orientation: &O
    ) -> Vec<(usize, BeaconBasedOrientation)> {
        let cur_sensor = &self.sensors[cur_sensor_idx];
        let mut nearby_sensors: Vec<(usize, BeaconBasedOrientation)> = vec![];
        for sensor_idx in 0..self.sensors.len() {
            if cur_sensor_idx != sensor_idx {
                let detection = cur_sensor.detect_orientation(&self.sensors[sensor_idx], cur_orientation);
                if detection.is_some() {
                    let orientation = detection.unwrap();
                     nearby_sensors.push((sensor_idx, orientation));
                 }
            };
        }
        nearby_sensors
    }

    fn align_sensors(&self) {
        // sensor zero is always located at 0,0, and oriented "correctly"
        let mut observed = HashSet::new();
        let mut stack: Vec<(usize, Vec3, O)> = vec![
            (0usize /* sensor idx */, (0,0,0), O::_1 /* orientation */)
        ];
        observed.insert(0);
        let mut true_beacon_positions: HashSet<Vec3> = HashSet::new();
        for beacon in &self.sensors[0].raw_readings {
            true_beacon_positions.insert(*beacon);
        }
        let mut scanner_positions: Vec<Vec3> = vec![(0,0,0)];
        while stack.len() > 0 {
            let (cur_sensor_idx, cur_sensor_pos, cur_sensor_orientation) = stack.pop().unwrap();
            let next_detections = self.detect_nearby_sensors(cur_sensor_idx, &cur_sensor_orientation);
            for (sensor_idx, det) in next_detections {
                if !observed.contains(&sensor_idx) {
                    observed.insert(sensor_idx);
                    // println!("[A] Detected sensor {} at {:?} relative to {}", sensor_idx, det.norm_sensor_pos, cur_sensor_idx);
                    let sensor_position = add_vec3s(&cur_sensor_pos, &det.norm_sensor_pos); 
                    scanner_positions.push(sensor_position);
                    println!("Detected sensor {} at {:?} relative to {}", sensor_idx, sensor_position, cur_sensor_idx);
                    for beacon in det.norm_positions {
                        // println!("{:?}",reoriented_beacon);
                        true_beacon_positions.insert(add_vec3s(&cur_sensor_pos, &beacon));
                    }
                    stack.push((sensor_idx, sensor_position, det.orientation));
                }
            }
        }
        println!("--");
        // for b in &true_beacon_positions {
        //     println!("{},{},{}", b.0, b.1, b.2);
        // }
        println!("Num beacons {}", true_beacon_positions.len());
        let quantify = |v: &Vec3| { v.0 + v.1 + v.2 };
        let diff = |a: &Vec3, b: &Vec3| { 
            (a.0 - b.0).abs() +
            (a.1 - b.1).abs() +
            (a.2 - b.2).abs()
        };
        let mut max = 0;
        // brute force since by now our brain doesn't work
        for i in 0..scanner_positions.len() {
            for j in 0..scanner_positions.len() {
                let d = diff(&scanner_positions[i], &scanner_positions[j]);
                if d > max {
                    max = d;
                }
            }
        }
        println!("Max sensor distance {}", max);
    }
}



struct BeaconBasedOrientation {
    orientation: O,
    norm_positions: Vec<Vec3>,
    norm_sensor_pos: Vec3
}

 impl Sensor {

    fn detect_orientation(&self, other: &Sensor, cur_orientation: &O) -> Option<BeaconBasedOrientation> {
        for (lhs_root_beacon_idx, left_per_o_offsets) in &self.relative_beacon_coords_per_beacon_per_o {
            // do have to try every left hand beacon as base point of reference
            // but not every orientation, only compare to original orientation
            let (_, lhs_offsets) = left_per_o_offsets.iter().find(|tup| {
                tup.0 == *cur_orientation
            }).unwrap();
            for (_, right_per_o_offsets) in &other.relative_beacon_coords_per_beacon_per_o {
                for  (o, rhs_offsets) in right_per_o_offsets {
                    let mut score = 0;
                    for rhs_beacon in rhs_offsets {
                        for lhs_beacon in lhs_offsets {
                            if lhs_beacon.offset_from_root == rhs_beacon.offset_from_root {
                                score += 1;
                                if score >= 12 {
                                    break;
                                }
                            }
                        }
                    }
                    if score >= 12 {
                        // println!("found {} for orientation {:?}", score, o);
                        // positions = offset + lhs root pos
                        // sensor pos = - rhs root pos
                        // recalculate positions of all beacons now
                        let rhs_root_beacon_idx = rhs_offsets[0].root_index;
                        let rhs_root_position: Vec3 = 
                            apply_orientation_to_point(o, &other.raw_readings[rhs_root_beacon_idx]);
                        let lhs_root_position: Vec3 =  apply_orientation_to_point(&cur_orientation,&self.raw_readings[*lhs_root_beacon_idx]);
                        let sensor_pos = 
                            (
                                -rhs_root_position.0 + lhs_root_position.0,
                                -rhs_root_position.1 + lhs_root_position.1,
                                -rhs_root_position.2 + lhs_root_position.2
                            );
                            
                        return Some(BeaconBasedOrientation {
                            orientation: *o,
                            norm_sensor_pos: sensor_pos,
                            norm_positions: other.raw_readings.iter().map(|d| {
                                let oriented_pos = apply_orientation_to_point(o, d);
                                (
                                    sensor_pos.0 + oriented_pos.0,
                                    sensor_pos.1 + oriented_pos.1,
                                    sensor_pos.2 + oriented_pos.2
                                )
                            }).collect()
                        })
                    }
                }
            }
        }
        None
    }
}

// axis' to rotate around, can rotate 90 degs, 12 / 3 / 6 / 9 o clock orientations
// about x
// about y
// about z
// about x,y
// about y,z
// about z,x

fn main() {
    let input_res = Problem::from_file("./src/input.txt");
    match input_res {
        Ok(mut input) => {
            input.align_sensors();
            // 680 is wrong
        }
        Err(e) => println!("{}", e),
    }
}
