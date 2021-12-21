use std::collections::HashSet;

struct TargetArea {
    x: [i32; 2],
    y: [i32; 2],
}

#[derive(Debug)]
struct Probe {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32
}

fn main() {

    // let input = TargetArea{x: [20,30], y: [-10,-5]};
    let input = TargetArea{x: [70,125], y: [-159,-121]};

    let mut velocities: HashSet<[i32; 2]> = HashSet::new();
    let mut hit_ypos: Vec<i32> = vec![];
    
    //12561
    for vx in -500..500 {
        for vy in -1000..1000 {

            let mut probe = Probe{x: 0, y: 0, vx: vx, vy: vy};
            let mut ypos: Vec<i32> = vec![];

            loop {
                ypos.push(probe.y);
                
                if verify(&probe, &input) {
                    let max_ypos = ypos.iter().max().unwrap();
                    println!("Sucess {} {} => {}", vx, vy, max_ypos);
                    velocities.insert([vx,vy]);
                    hit_ypos.push(*max_ypos);
                    break;
                }

                if out_bounds(&probe, &input) {
                    break;
                }

                step(&mut probe);
            }

        }
    }

    println!("Max y: {}", hit_ypos.iter().max().unwrap());
    println!("Number of velocities: {}", velocities.len());

}

fn step(probe: &mut Probe) {
    probe.x = probe.x + probe.vx;
    probe.y = probe.y + probe.vy;
    probe.vx -= if probe.vx > 0 { 1 } else { if probe.vx == 0 { 0 } else { -1 }};
    probe.vy -= 1;
}

fn verify(probe: &Probe, target: &TargetArea) -> bool {
    return target.x[0] <= probe.x && probe.x <= target.x[1] 
        && target.y[0] <= probe.y && probe.y <= target.y[1];
}

fn out_bounds(probe: &Probe, target: &TargetArea) -> bool {
    return probe.x > target.x[1] || probe.y < target.y[0];
}