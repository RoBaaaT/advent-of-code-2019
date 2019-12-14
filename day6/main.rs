use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug, Clone, PartialEq)]
struct Object {
    name: String,
    orbits: String
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let input = BufReader::new(&input_file);

    let mut objects = Vec::new();

    for line in input.lines() {
        let obj = line.unwrap();
        let object_parts: Vec<&str> = obj.split(')').collect();
        assert_eq!(object_parts.len(), 2);
        objects.push(Object { name: object_parts[1].to_string(), orbits: object_parts[0].to_string() });
    }

    let mut total_orbits = 0;
    for i in 0..objects.len() {
        let mut j = i;
        while j != std::usize::MAX {
            let id = j;
            j = std::usize::MAX;
            total_orbits += 1;
            for k in 0..objects.len() {
                if objects[k].name == objects[id].orbits {
                    j = k;
                    break;
                }
            }
        }
    }
    println!("Part 1: {}", total_orbits);

    let mut unvisited_objects = objects.clone();
    let mut reachable_objects = vec![objects.iter().find(|&x| x.name == "YOU").unwrap().clone()];
    unvisited_objects.remove(unvisited_objects.iter().position(|x| x.name == "YOU").unwrap());
    let mut distance = 0;
    loop {
        let mut new_reachable_objects = Vec::new();
        for object in &reachable_objects {
            if let Some(pos) = unvisited_objects.iter().position(|x| x.name == object.orbits) {
                new_reachable_objects.push(unvisited_objects[pos].clone());
                unvisited_objects.remove(pos);
            };
            while let Some(pos) = unvisited_objects.iter().position(|x| x.orbits == object.name) {
                new_reachable_objects.push(unvisited_objects[pos].clone());
                unvisited_objects.remove(pos);
            };
        }
        reachable_objects = new_reachable_objects;
        if let Some(_) = reachable_objects.iter().find(|x| x.name == "SAN") {
            break;
        }
        distance += 1;
    }
    println!("Part 2: {}", distance - 1); // - 1 because the distance includes the first orbit
}