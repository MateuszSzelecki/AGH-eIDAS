pragma circom 2.0.0; 

include "node_modules/circomlib/circuits/comparators.circom";
include "node_modules/circomlib/circuits/poseidon.circom";


template StudentStatusVerifier() {
    // To dajemy w input.json
    signal input DateOfBirth;     // timestamp urodzin  w sekundach
    signal input isActiveStudent; // status studenta: 1 = tak, 0 = nie
    
    signal input DateToday;
    signal input nonce;


    signal output isValid; // To co zworci circuit
    signal output nonceEcho;

    signal ageInSeconds;
    ageInSeconds <== DateToday - DateOfBirth;

    component ageCheck = GreaterEqThan(64); // gotowy component dla 64 bitowych liczb
    ageCheck.in[0] <== ageInSeconds;
    ageCheck.in[1] <== 567648000; // hardcoded 18 lat w sekundach  


    // na wejsciu tylko 0 lub 1
    isActiveStudent * (isActiveStudent - 1) === 0;

    // bramka AND zadzaial tylko jesli 1 * 1
    isValid <== ageCheck.out * isActiveStudent;

    component echoHash = Poseidon(1);
    echoHash.inputs[0] <== nonce;
    nonceEcho <== echoHash.out;
}

component main = StudentStatusVerifier();
