fn opcode_run(inst: &mut [usize]) {
    let len = inst.len();
    let mut index = 0;
    loop {
        if index >= len {
            break;
        }

        match inst[index] {
            1 => {
                let dest = inst[index + 3];
                let op1 = inst[index + 1];
                let op2 = inst[index + 2];
                inst[dest] = inst[op1] + inst[op2];
                index += 4;
            }
            2 => {
                let dest = inst[index + 3];
                let op1 = inst[index + 1];
                let op2 = inst[index + 2];
                inst[dest] = inst[op1] * inst[op2];
                index += 4;
            }
            99 => break,
            _ => eprintln!("Wrong OpCode"),
        }
    }
}

fn reset_intcode(src: &[usize], dst: &mut [usize]) {
    for index in 0..src.len() {
	dst[index] = src[index];
    }
}

pub fn run() {
    let base = &[
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 9, 19, 23, 2, 23, 10, 27,
        1, 27, 5, 31, 1, 31, 6, 35, 1, 6, 35, 39, 2, 39, 13, 43, 1, 9, 43, 47, 2, 9, 47, 51, 1, 51,
        6, 55, 2, 55, 10, 59, 1, 59, 5, 63, 2, 10, 63, 67, 2, 9, 67, 71, 1, 71, 5, 75, 2, 10, 75,
        79, 1, 79, 6, 83, 2, 10, 83, 87, 1, 5, 87, 91, 2, 9, 91, 95, 1, 95, 5, 99, 1, 99, 2, 103,
        1, 103, 13, 0, 99, 2, 14, 0, 0,
    ];

    let vec_one = &mut base.clone();
    vec_one[1] = 12;
    vec_one[2] = 2;
    opcode_run(vec_one);
    println!("{:?}", vec_one[0]);

    // part two: determine what pair of inputs produces the output 19690720.
    let vec_two = &mut base.clone();
    for op1 in 1..99 {
        for op2 in 1..99 {
            vec_two[1] = op1;
            vec_two[2] = op2;
            opcode_run(vec_two);
            if vec_two[0] == 19690720 as usize {
                println!("Eureca, I found ({}, {}): {}", op1, op2, (100 * op1) + op2);
                return;
            }
	    reset_intcode(base, vec_two);
        }
    }
}
