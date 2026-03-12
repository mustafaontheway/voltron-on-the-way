address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun learn_move() {

            let age: u8 = 17;

            let year: u16 = 2026;

            let birth_year: u16 = year - (age as u16);

            print(&birth_year);
        }

        #[test]
        fun testing() {

            learn_move(); // [debug] 2009
        }
    }
}

// aptos move test

