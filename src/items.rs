
#[allow(non_upper_case_globals, non_snake_case)]
pub mod Item {

    #[derive(PartialEq)]
    pub struct Gun {
        pub name: &'static str,
        pub bullet_speed: f32,
        pub bullet_spread: f32,
        pub bullets_per_shot: u32,
    }

    pub struct Clothes {
        pub name: &'static str,
    }

    // Guns
    impl Gun {
        pub fn sawed_shotgun() -> Gun {
            Gun {
                name: "sawed_shotgun",
                bullet_speed: 6.5,
                bullet_spread: 0.15,
                bullets_per_shot: 8,
            }
        }
    
        pub fn sniper() -> Gun {
            Gun {
                name: "sniper",
                bullet_speed: 9.0,
                bullet_spread: 0.01,
                bullets_per_shot: 1
            }
        }
    }

    impl Clothes {
    // Clothes
    pub fn blue_clothes() -> Clothes {
        Clothes {
            name: "blue" 
        }
    }
    pub fn dark_clothes() -> Clothes {
        Clothes {
            name: "dark" 
        }
    }
    }
}

