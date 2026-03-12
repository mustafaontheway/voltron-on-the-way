address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun learn_move() {

            let diff: i8 = -12;

            let loss: i32 = -421_354;

            print(&diff);
            print(&loss);
        }

        #[test]
        fun testing() {

            learn_move();
        }
    }
}

// [debug] -12
// [debug] -421354

// aptos move test

