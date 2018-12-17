use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

const PATH_TO_INPUT: &'static str = "src/files/ten/example.txt";

#[derive(Debug)]
enum PointInSky {
    Star { position: (i32, i32), velocity: (i32, i32) },
    Empty,
}

impl FromStr for PointInSky {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = s.chars();

        let pos_x = iter.skip_while(|&c| c != '<')
            .take_while(|&c| c != ',');


//        println!("{}", pos_x);

        Err(())
    }
}

//type Sky = Vec<Vec<PointInSky>>;
//
//fn create_sky() -> Sky {
//    let stars = BufReader::new(File::open(PATH_TO_INPUT).unwrap()).lines()
//        .map(|line| PointInSky::from_str(&line.unwrap()));
//}

