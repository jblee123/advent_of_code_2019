use std::collections::HashMap;

const ROOT_NAME: &str = "COM";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpaceObj {
    name: String,
    parent: String,
    children: Vec<String>,
    num_orbits: u32,
}

impl SpaceObj {
    fn new(name: &str) -> SpaceObj {
        SpaceObj {
            name: name.to_string(),
            parent: "".to_string(),
            children: vec![],
            num_orbits: 0,
        }
    }
}

pub type OrbitSystem = HashMap<String, SpaceObj>;

fn do_calc_orbit_nums(orbits: &mut OrbitSystem, name: &str, dist: u32) {
    let children: Vec<String>;

    {
        let mut parent = orbits.get_mut(name).unwrap();
        parent.num_orbits = dist;
        children = parent.children.clone();
    }

    for child_name in children {
        do_calc_orbit_nums(orbits, &child_name, dist + 1);
    }
}

fn calc_orbit_nums(orbits: &mut OrbitSystem) {
    do_calc_orbit_nums(orbits, ROOT_NAME, 0);
}

fn get_parent_chain(orbits: &OrbitSystem, name: &str) -> Vec<String> {
    let parent = &orbits.get(name).unwrap().parent;
    let mut chain = vec![name.to_string()];
    if parent != "" {
        let parent_chain = get_parent_chain(orbits, parent);
        chain.extend_from_slice(&parent_chain[..]);
    }
    chain
}

pub fn get_num_transfers(orbits: &OrbitSystem, obj1: &str, obj2: &str) -> u32 {
    let mut chain1 = get_parent_chain(orbits, obj1);
    let mut chain2 = get_parent_chain(orbits, obj2);

    while !chain1.is_empty()
        && !chain2.is_empty()
        && chain1.last() == chain2.last()
    {
        chain1.pop();
        chain2.pop();
    }

    (chain1.len() + chain2.len() - 2) as u32
}

pub fn parse_input(input: &str) -> OrbitSystem {
    let mut sys = OrbitSystem::new();

    // scan once just for objects
    input.lines().for_each(|line| {
        let mut parts = line.split(")"); 
        let parent = parts.next().unwrap();
        if parent == ROOT_NAME {
            sys.insert(parent.to_string(), SpaceObj::new(parent));
        }
        let child = parts.next().unwrap();
        sys.insert(child.to_string(), SpaceObj::new(child));
    });

    // now do parent/child connections
    input.lines().for_each(|line| {
        let mut parts = line.split(")"); 
        let parent = parts.next().unwrap();
        let child = parts.next().unwrap();
        
        sys.get_mut(child).unwrap().parent = parent.to_string();
        sys.get_mut(parent).unwrap().children.push(child.to_string());
    });

    calc_orbit_nums(&mut sys);

    sys
}

fn do_get_total_orbits(orbits: &OrbitSystem, obj_name: &str) -> u32 {
    let obj = orbits.get(obj_name).unwrap();
    let mut num_orbits = obj.num_orbits;

    for child_name in &obj.children {
        num_orbits += do_get_total_orbits(orbits, &child_name);
    }

    num_orbits
}

pub fn get_total_orbits(orbits: &OrbitSystem) -> u32 {
    do_get_total_orbits(orbits, ROOT_NAME)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_orbits_basics() {
        use super::*;

        let input = concat!(
            "D)E\n",
            "E)F\n",
            "B)C\n",
            "B)G\n",
            "C)D\n",
            "G)H\n",
            "D)I\n",
            "COM)B\n",
            "E)J\n",
            "J)K\n",
            "K)L\n",
        );

        let orbit_sys = parse_input(&input);
        assert_eq!(orbit_sys.len(), 12);

        let obj = orbit_sys.get("COM").unwrap();
        assert_eq!(obj.name, "COM");
        assert_eq!(obj.num_orbits, 0);
        assert_eq!(obj.parent, "");
        assert_eq!(obj.children.len(), 1);
        assert_eq!(obj.children.contains(&"B".to_string()), true);

        let obj = orbit_sys.get("B").unwrap();
        assert_eq!(obj.name, "B");
        assert_eq!(obj.num_orbits, 1);
        assert_eq!(obj.parent, "COM");
        assert_eq!(obj.children.len(), 2);
        assert_eq!(obj.children.contains(&"C".to_string()), true);
        assert_eq!(obj.children.contains(&"G".to_string()), true);

        let obj = orbit_sys.get("L").unwrap();
        assert_eq!(obj.name, "L");
        assert_eq!(obj.num_orbits, 7);
        assert_eq!(obj.parent, "K");
        assert_eq!(obj.children.len(), 0);

        let result = get_total_orbits(&orbit_sys);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_get_num_transfers() {
        use super::*;

        let input = concat!(
            "COM)B\n",
            "B)C\n",
            "C)D\n",
            "D)E\n",
            "E)F\n",
            "B)G\n",
            "G)H\n",
            "D)I\n",
            "E)J\n",
            "J)K\n",
            "K)L\n",
            "K)YOU\n",
            "I)SAN\n",
        );

        let orbit_sys = parse_input(&input);
        let result = get_num_transfers(&orbit_sys, "YOU", "SAN");
        assert_eq!(result, 4);
    }
}
