/*
Let's get to work
Recall that we need to track all the Spiders created from spider_nest. We can do this by declaring two new structs:

Spider which defines a Spider. This is already done in the code for you.
A new resource struct named SpiderSwarm which has a vector of Spider structs. Remember this resource needs to be stored at 0xcafe in init_module with an empty vector of Spiders to start with.

*/

module 0xcafe::spider_nest {
    use std::vector;

    struct SpiderDna has key {
        dna_digits: u64,
        dna_modulus: u256,
    }

    struct Spider has store {
        dna: u64,
    }

    struct SpiderSwarm has key { 
        spiders: vector<Spider>,
    }

    fun init_module(cafe_signer: &signer) {
        let dna_digits = 10;
        let dna_modulus = 10 ^ dna_digits;
        move_to(cafe_signer, SpiderDna {
            dna_digits,
            dna_modulus: (dna_modulus as u256),
        });

        move_to(cafe_signer, SpiderSwarm {
            spiders: vector[],
        });

    }
}
