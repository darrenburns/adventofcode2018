use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::HashMap;

const PRUNE_HEIGHT: i32 = 15;
const PRUNE_WIDTH: i32 = 100;
const PATH_TO_INPUT: &'static str = "src/files/ten/stars.txt";

#[derive(Debug, Copy, Clone)]
struct Star { position: (i32, i32), velocity: (i32, i32) }

type Sky = Vec<Vec<bool>>;

impl FromStr for Star {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos_x, pos_y, vel_x, vel_y) =
            scan_fmt!(s, "position=< {d},  {d}> velocity=<{d},  {d}>", i32, i32, i32, i32);

        Ok(Star {
            position: (pos_x.unwrap(), pos_y.unwrap()),
            velocity: (vel_x.unwrap(), vel_y.unwrap()),
        })
    }
}

fn print_sky(sky: &Sky) {
    let sky_lines = sky.iter()
        .map(|row| row.iter().map(|&b| if b { "#" } else { "." }).collect::<String>());

    for line in sky_lines {
        println!("{}", line);
    }
}


pub fn find_message() {
    let mut stars: Vec<Star> = BufReader::new(File::open(PATH_TO_INPUT).unwrap()).lines()
        .map(|line| Star::from_str(&line.unwrap()).unwrap())
        .collect();

    let sky = create_sky_of(&stars);
    if let Some(sky) = sky {
        print_sky(&sky);
    }

    for second in 0..20000 {
        // Update the stars based on how many seconds have passed
        let new_stars = stars.iter().map(|&star| Star {
            position: (star.position.0 + star.velocity.0 * second, star.position.1 + star.velocity.1 * second),
            ..star
        }).collect();

        let sky = create_sky_of(&new_stars);

        if let Some(sky) = sky {
            println!("Second: {}", second);
            print_sky(&sky);
            break;
        }
    }
}


fn create_sky_of(stars: &Vec<Star>) -> Option<Sky> {
    let minimum_x = stars.iter().min_by_key(|star| star.position.0).expect("Cannot find minimum x position").position.0;
    let minimum_y = stars.iter().min_by_key(|star| star.position.1).expect("Cannot find minimum y position").position.1;
    let maximum_x = stars.iter().max_by_key(|star| star.position.0).expect("Cannot find minimum x position").position.0;
    let maximum_y = stars.iter().max_by_key(|star| star.position.1).expect("Cannot find minimum y position").position.1;

    let height = maximum_y - minimum_y + 1;
    let width = maximum_x - minimum_x + 1;

    // Prune out large skies since they won't contain a message
    if height > PRUNE_HEIGHT || width > PRUNE_WIDTH {
        return None;
    }
    Some(stars.iter()
        .fold(vec![vec![false; width as usize]; height as usize], |mut sky, star| {
            let normalised_y = star.position.1 - minimum_y;
            let normalised_x = star.position.0 - minimum_x;
            sky[normalised_y as usize][normalised_x as usize] = true;
            sky
        }))
}


// Notes on how we could determine programmatically whether the current star layout may form a message:

// If the bounding box size is minimal, it means stars are grouped and there's a high probability that a message has been formed.

// If the average manhattan distance between the stars is below some threshold, it's a candidate for containing the solution message.

// Another possible solution is to do a recursive paint fill starting from one of the stars, and see how many stars we encounter along the way
// if we can hop star-to-star many times, we're inside a clump of stars and almost certainly inside a letter