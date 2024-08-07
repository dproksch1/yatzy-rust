
pub const FIELD_CNT: u8 = 13;

pub struct Data {
    ones: i8,
    twos: i8,
    threes: i8,
    fours: i8,
    fives: i8,
    sixes: i8,
    threeofakind: i8,
    fourofakind: i8,
    smallstraight: i8,
    largestraight: i8,
    fullhouse: i8,
    yatzy: i8,
    chance: i8,
}

impl Data {
    pub fn new() -> Self {
        Self {
            ones: -1,
            twos: -1,
            threes: -1,
            fours: -1,
            fives: -1,
            sixes: -1,
            threeofakind: -1,
            fourofakind: -1,
            smallstraight: -1,
            largestraight: -1,
            fullhouse: -1,
            yatzy: -1,
            chance: -1,
        }
    }


    fn count_instances(dice: &Vec<u8>, inst: u8) -> i8 {
        let mut n: i8 = 0;
        for i in 0..5 {
            if dice[i] == inst {
                n += 1;
            }
        }
        n
    }

    pub fn get_numbers_sum(&self) -> u32 {
        self.get_ones() + self.get_twos() + self.get_threes() + self.get_fours() + self.get_fives() + self.get_sixes()
    }

    pub fn add_ones(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.ones >= 0 {
            return Err("Ones field is already set.")
        }

        self.ones = Self::count_instances(&dice, 1);
        Ok(())
    }

    pub fn get_ones(&self) -> u32 {
        if self.ones >= 0 {self.ones as u32} else {0}
    }

    pub fn add_twos(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.twos >= 0 {
            return Err("Twos field is already set.")
        }

        self.twos = Self::count_instances(&dice, 2) * 2;
        Ok(())
    }

    pub fn get_twos(&self) -> u32 {
        if self.twos >= 0 {self.twos as u32} else {0}
    }

    pub fn add_threes(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.threes >= 0 {
            return Err("Threes field is already set.")
        }

        self.threes = Self::count_instances(&dice, 3) * 3;
        Ok(())
    }

    pub fn get_threes(&self) -> u32 {
        if self.threes >= 0 {self.threes as u32} else {0}
    }

    pub fn add_fours(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.fours >= 0 {
            return Err("Fours field is already set.")
        }

        self.fours = Self::count_instances(&dice, 4) * 4;
        Ok(())
    }

    pub fn get_fours(&self) -> u32 {
        if self.fours >= 0 {self.fours as u32} else {0}
    }

    pub fn add_fives(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.fives >= 0 {
            return Err("Fives field is already set.")
        }

        self.fives = Self::count_instances(&dice, 5) * 5;
        Ok(())
    }

    pub fn get_fives(&self) -> u32 {
        if self.fives >= 0 {self.fives as u32} else {0}
    }

    pub fn add_sixes(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.sixes >= 0 {
            return Err("Sixes field is already set.")
        }

        self.sixes = Self::count_instances(&dice, 6) * 6;
        Ok(())
    }

    pub fn get_sixes(&self) -> u32 {
        if self.sixes >= 0 {self.sixes as u32} else {0}
    }    

    pub fn add_threeofakind(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.threeofakind >= 0 {
            return Err("Three of a kind field is already set.")
        }

        let mut sum: u8 = 0;

        for i in 0..3 {
            if Self::count_instances(&dice, dice[i]) >= 3 {
                sum = dice.iter().sum();
                break;
            }
        }

        self.threeofakind = sum as i8;

        Ok(())
    }

    pub fn get_threeofakind(&self) -> u32 {
        if self.threeofakind >= 0 {self.threeofakind as u32} else {0}
    }    

    pub fn add_fourofakind(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.fourofakind >= 0 {
            return Err("Four of a kind field is already set.")
        }

        let mut sum: u8 = 0;

        for i in 0..2 {
            if Self::count_instances(&dice, dice[i]) >= 4 {
                sum = dice.iter().sum();
                break;
            }
        }

        self.fourofakind = sum as i8;

        Ok(())
    }

    pub fn get_fourofakind(&self) -> u32 {
        if self.fourofakind >= 0 {self.fourofakind as u32} else {0}
    }    

    pub fn add_smallstraight(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.smallstraight >= 0 {
            return Err("Small straight field is already set.")
        }

        let mut straight: u8 = 0;

        for i in 1..=6 {
            if Self::count_instances(&dice, i) > 0 {
                straight += 1;
            } else if i < 3 {
                straight = 0;
            } else if i > 4 {
                break;
            }
        }

        self.smallstraight = if straight >= 4 {30} else {0};

        Ok(())
    }

    pub fn get_smallstraight(&self) -> u32 {
        if self.smallstraight >= 0 {self.smallstraight as u32} else {0}
    }    

    pub fn add_largestraight(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.largestraight >= 0 {
            return Err("Large straight field is already set.")
        }

        let mut straight: u8 = 0;

        for i in 1..=6 {
            if Self::count_instances(&dice, i) > 0 {
                straight += 1;
            } else if i != 1 && i != 6 {
                straight = 0;
                break;
            }
        }
 
        self.largestraight = if straight >= 5 {40} else {0};

        Ok(())
    }

    pub fn get_largestraight(&self) -> u32 {
        println!("{}", self.largestraight);
        if self.largestraight >= 0 {self.largestraight as u32} else {0}
    }

    pub fn add_fullhouse(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.fullhouse >= 0 {
            return Err("Full house field is already set.")
        }

        let k1 = dice[0];
        let mut k2 = 0;

        let mut n1: u8 = 1;
        let mut n2: u8 = 0;

        for i in 1..5 {
            match dice[i] {
                k if k == k1 => n1 += 1,
                k if k == k2 => n2 += 1,
                _ => {
                        k2 = dice[i];
                        n2 = 1;
                    }
            }
        }

        if (n1 == 3 && n2 == 2) || (n1 == 2 && n2 == 3) {
            self.fullhouse = 25;
        } else {
            self.fullhouse = 0;
        }


        Ok(())
    }

    pub fn get_fullhouse(&self) -> u32 {
        if self.fullhouse >= 0 {self.fullhouse as u32} else {0}
    }

    pub fn add_yatzy(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.yatzy >= 0 {
            return Err("Yatzy field is already set.")
        }

        if Self::count_instances(&dice, dice[0]) == 5 {
            self.yatzy = 50;
        } else {
            self.yatzy = 0;
        }

        Ok(())
    }

    pub fn get_yatzy(&self) -> u32 {
        if self.yatzy >= 0 {self.yatzy as u32} else {0}
    }

    pub fn add_chance(&mut self, dice: &Vec<u8>) -> Result<(), &str> {

        if self.chance >= 0 {
            return Err("Chance field is already set.")
        }

        self.chance = dice.iter().sum::<u8>() as i8;

        Ok(())
    }

    pub fn get_chance(&self) -> u32 {
        if self.chance >= 0 {self.chance as u32} else {0}
    }

    pub fn fetch_score(&self) -> u32 {
        let mut nums = self.get_numbers_sum();
        if nums >= 63 {
            nums += 50;
        }
        nums + self.get_threeofakind() + self.get_fourofakind() + self.get_smallstraight()
             + self.get_largestraight() + self.get_fullhouse() + self.get_yatzy() + self.get_chance()
    }

    pub fn print_data(&self) {
        println!("ones: {}", if self.ones >= 0 {self.ones} else {-44});
        println!("twos: {}", if self.twos >= 0 {self.twos} else {-44});
        println!("threes: {}", if self.threes >= 0 {self.threes} else {-44});
        println!("fours: {}", if self.fours >= 0 {self.fours} else {-44});
        println!("fives: {}", if self.fives >= 0 {self.fives} else {-44});
        println!("sixes: {}", if self.sixes >= 0 {self.sixes} else {-44});
        println!("bonus: {}", if self.get_numbers_sum() >= 63 {50} else {-66});
        println!("threeofakind: {}", if self.threeofakind >= 0 {self.threeofakind} else {-44});
        println!("fourofakind: {}", if self.fourofakind >= 0 {self.fourofakind} else {-44});
        println!("smallstraight: {}", if self.smallstraight >= 0 {self.smallstraight} else {-44});
        println!("largestraight: {}", if self.largestraight >= 0 {self.largestraight} else {-44});
        println!("fullhouse: {}", if self.fullhouse >= 0 {self.fullhouse} else {-44});
        println!("yatzy: {}", if self.yatzy >= 0 {self.yatzy} else {-44});
        println!("chance: {}\n", if self.chance >= 0 {self.chance} else {-44});
    }
}
