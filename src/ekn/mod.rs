use euclidian_rythms::*;
use gcd::binary_u32;

pub mod ui;

pub enum EknType {
    All,
    Euclidian,
    EuclidianNoTrivial,
}

/*
next time: create trait with key and velocity
*/
pub struct Ekn {
    events: u32, // Pulsation k
    steps: u32,  // Duration n
    rotation: u32,
    gcd_cache: Vec<u32>,    // Example All = [1, 2, 3, 4, 5, 6, 7] ; E(*,8,*) = [1, 3, 5, 7] ; E(*,8,*) without trival = [3, 5]
    pattern_cache: Vec<u8>, // Example E(3,8,0) = [1, 0, 0, 1, 0, 0, 1, 0]
    ekn_type: EknType,
    current_step: u32, // current step
}

impl Default for Ekn {
    fn default() -> Self {
        let mut ekn = Self {
            events: 3,
            steps: 8,
            rotation: 0,
            gcd_cache: vec![],
            pattern_cache: vec![],
            ekn_type: EknType::Euclidian,
            current_step: 0,
        };
        ekn.update_gcd_cache();
        ekn.update_pattern_cache();
        ekn
    }
}

impl Ekn {
    pub fn new(k: u32, n: u32) -> Self {
        let mut ekn = Self::default();
        ekn.set_steps(n);
        ekn.set_events(k);
        ekn
    }

    /* cache */

    fn update_gcd_cache(&mut self) {
        self.set_current_step(0); // reset seek
        self.gcd_cache = match self.ekn_type {
            EknType::All => {
                (1..self.steps).collect()
            },
            _ => {
                // get all pgcd([1..steps], steps) == 1
                (1..self.steps)
                    .filter(|x| binary_u32(*x, self.steps) == 1)
                    .collect()
            },
        };
        if let EknType::EuclidianNoTrivial = self.ekn_type {
            if self.gcd_cache.len() > 2 {
                self.gcd_cache.remove(0);
                self.gcd_cache.remove(self.gcd_cache.len()-1);
            }
        }
        // println!("update_gcd_cache({}) == {:?}", self.steps, self.gcd_cache);
    }

    fn update_pattern_cache(&mut self) {
        self.set_current_step(0); // reset seek
        self.pattern_cache = vec![0; self.steps as usize];
        euclidian_rythm(&mut self.pattern_cache.as_mut(), self.events as usize).unwrap();
        self.pattern_cache.rotate_right(self.rotation as usize);
        // println!("update_pattern_cache() {}/{} -> {:?}", self.events, self.steps, self.pattern_cache);
    }

    /** Change Euclidian Type */
    pub fn set_euclidian(&mut self, ekn_type: EknType) -> u32 {
        self.ekn_type = ekn_type;
        self.update_gcd_cache();
        self.set_events(1); // nearest
        self.update_pattern_cache();
        return self.events;
    }

    /** Get Ekn pattern  */
    pub fn get_pattern(&self) -> Vec<u8> {
        return self.pattern_cache.clone();
    }

    /* events */

    pub fn get_events_list(&self) -> Vec<u32> {
        return self.gcd_cache.clone();
    }

    pub fn get_nearest_euclidian_events(&mut self, events: u32) -> u32 {
        return match self.gcd_cache.iter().position(|x| x >= &events) {
            Some(i) => self.gcd_cache[i],
            None => {
                let len = self.gcd_cache.len();
                return if len == 0 { 0 } else { self.gcd_cache[len - 1] };
            }
        };
    }

    pub fn get_events(&self) -> u32 {
        self.events
    }

    pub fn set_events(&mut self, events: u32) -> u32 {
        self.events = self.get_nearest_euclidian_events(events);
        self.update_pattern_cache();
        return self.events;
    }

    /* step */

    pub fn get_current_step(&self) -> u32 {
        return self.current_step;
    }

    pub fn set_current_step(&mut self, step: u32) -> u32 {
        self.current_step = step % self.steps;
        return self.current_step;
    }

    pub fn next_step(&mut self) -> u32 {
        self.set_current_step(self.current_step + 1)
    }

    /* steps */

    pub fn is_event(&self) -> bool {
        self.pattern_cache[self.current_step as usize] == 1
    }

    pub fn get_steps(&self) -> u32 {
        self.steps
    }

    pub fn set_steps(&mut self, steps: u32) -> (u32, u32) {
        if steps <= 1 {
            self.steps = 2;
        } else {
            self.steps = steps;
        }
        self.update_gcd_cache();
        self.events = self.get_nearest_euclidian_events(self.events);
        self.update_pattern_cache();
        return (self.events, self.steps);
    }

    /* rotation */

    pub fn get_rotation(&self) -> u32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: u32) -> u32 {
        let r = if rotation < self.steps {
            rotation
        } else {
            self.steps - 1
        };
        self.rotation = r;
        self.update_pattern_cache();
        return r;
    }
}
