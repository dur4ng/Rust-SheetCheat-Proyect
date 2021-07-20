// 1. Bring the macros error, warn, info, debug, and trace into scope from the log package with a
// `use` statement.
//
// - You should be able to run `cargo build --lib` successfully after each step.
//
// Hint: You may need to update Cargo.toml first

use log::{error, warn, info, debug, trace};

#[derive(Debug)]
pub struct Frog {
    energy: u8,
    sleeping: bool,
}

impl Frog {
    pub fn new() -> Self {
        // 2. Use debug!() to log "A new Frog has been created"
        debug!("A new Frog has been created");
        Default::default()
    }
    pub fn hop(&mut self) {
        // 3. Use info!() to log that a Frog hopped, and how much energy is left
        
        self.energy -= 1;
        info!("A frog hopped! It has {} energy left.", self.energy);
        if self.energy == 0 {
            // 4. Use warn!() to warn that the frog will go to sleep since he ran out of energy
            warn!("Frog ran out of energy. Going to sleep now. zzzzz");
            self.sleep();
        }
    }
    // info, error
    pub fn sleep(&mut self) {
        if !self.sleeping {
            // 5. Use error!() to log a (non-fatal) error stating that the Frog is already asleep
            error!("Frog is already asleep!");
            self.sleeping = true;
        }
    }
}

impl Default for Frog {
    fn default() -> Self {
        // 6. Use trace!() to log that a default value was generated, with the debug representation
        let frog = Frog {
            energy: 5,
            sleeping: false,
        };
        trace!("Created a default Frog: {:?}", frog);
        frog
    }
}
