address my_temp_addr {

    module Daily {

        //use std::debug::print;

        fun learn_move() {

            let _age: u8 = 17;
            let _year: u16 = 2026;
            let _expected_yearly_sales: u64 = 320_412_500;
        }

        #[test]
        fun testing() {

            learn_move();
        }
    }
}
