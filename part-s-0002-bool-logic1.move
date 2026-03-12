address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun learn_move() {

            let x: u8 = 20;
            let y: u8 = 12;
            let z: u8 = 8;

            let cond1: bool = (x > y) && (y != z);
            let cond2: bool = (x == y) || (!(x == z));

            print(&cond1);
            print(&cond2);
        }

        #[test]
        fun testing() {

            learn_move();
        }
    }
}

// [debug] true
// [debug] true


// aptos move test

