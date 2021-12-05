use std::fmt;

pub struct Level {
    level: u16,
    xp: u32,
    level_fn: fn(u16) -> u32,
}

impl Level {
    fn level_up(&mut self, xp: u32) {
        self.xp -= xp;
        self.level += 1;
    }

    fn convert_xp_into_level(&mut self) -> bool {
        let mut leveled = false;
        let mut xp_requirment = 0;
        while {
            xp_requirment = self.lvl_fn();
            self.xp >= xp_requirment
        } {
            leveled = true;
            self.level_up(xp_requirment)
        }
        leveled
    }

    fn lvl_fn(&self) -> u32 {
        (self.level_fn)(self.level)
    }
}

impl Level{
    pub fn new(level_fn: fn(u16) -> u32) -> Self {
        Self{
            level: 1,
            xp: 0,
            level_fn
        }
    }

    pub fn add_xp(&mut self, xp: u32) -> bool {
        let mut leveled = false;
        if (u32::MAX - xp) < self.xp {
            panic!("to be considered later [xp add]")
        }
        self.xp += xp;
        leveled = self.convert_xp_into_level() | leveled;
        leveled
    }

    pub fn get_xp(&self) -> u32 {
        self.xp
    }

    pub fn get_level(&self) -> u16 {
        self.level
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Level: {}, Xp: {}/{}",
            self.level,
            self.xp,
            self.lvl_fn()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Level;
    #[test]
    fn basic() {
        let mut l = Level::new(|level| {
            (level as u32).pow(2) * 100
        });
        l.add_xp(200);
    }
}
