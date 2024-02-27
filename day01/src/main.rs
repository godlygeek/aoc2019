use std::fs;

fn read_masses() -> Vec<i32> {
    let text = fs::read_to_string("input.txt").expect("Reading input failed");
    let mut masses: Vec<i32> = Vec::new();
    for line in text.lines() {
        masses.push(line.parse().unwrap());
    }
    masses
}

fn fuel_required_for_mass_naive(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_required_for_mass_recursive(mut mass: i32) -> i32 {
    let mut ret = 0;
    loop {
        let fuel_mass = fuel_required_for_mass_naive(mass);
        if fuel_mass <= 0 {
            break;
        }
        ret += fuel_mass;
        mass = fuel_mass;
    }
    ret
}

fn part1(masses: &Vec<i32>) {
    let mut total_fuel = 0;
    for mass in masses {
        total_fuel += fuel_required_for_mass_naive(*mass);
    }
    println!("Part 1: {}", total_fuel);
}

fn part2(masses: &Vec<i32>) {
    let mut total_fuel = 0;
    for mass in masses {
        total_fuel += fuel_required_for_mass_recursive(*mass);
    }
    println!("Part 2: {}", total_fuel);
}

fn main() {
    let masses = read_masses();
    part1(&masses);
    part2(&masses);
}
