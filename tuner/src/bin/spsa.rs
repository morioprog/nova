use newbot::evaluator::BUILD;
use nova_tuner::simulate::select_best_evaluator;
use rand::Rng;

fn main() {
    let mut eval = BUILD;

    let constrain_positive = |x: i32| x.max(0);
    let constrain_negative = |x: i32| x.min(0);

    let ptrs = [
        (&mut eval.bump as *mut i32, -1, 20),
        (&mut eval.dent as *mut i32, -1, 20),
        // (&mut eval.dead_cells as *mut i32, -1, 20),
        (&mut eval.conn_2 as *mut i32, 1, 10),
        (&mut eval.conn_3 as *mut i32, 1, 10),
        // (&mut eval.non_u_shape as *mut i32, -1, 10),
        // (&mut eval.non_u_shape_sq as *mut i32, -1, 10),
        // (&mut eval.frame as *mut i32, -1, 10),
        // (&mut eval.frame_by_chain as *mut i32, -1, 10),
        // (&mut eval.frame_by_chigiri as *mut i32, -1, 10),
    ];

    for i in 1..=100 {
        println!("> SPSA iteration {}", i);

        for ptr in ptrs {
            let delta = rand::thread_rng().gen_range(2..=ptr.2);

            let w_org = eval.clone();

            let w_pos = unsafe {
                match ptr.1 {
                    1 => *ptr.0 = constrain_positive(*ptr.0 + delta),
                    -1 => *ptr.0 = constrain_negative(*ptr.0 + delta),
                    _ => unreachable!(),
                }
                eval.clone()
            };

            eval = w_org;
            let w_neg = unsafe {
                match ptr.1 {
                    1 => *ptr.0 = constrain_positive(*ptr.0 - delta),
                    -1 => *ptr.0 = constrain_negative(*ptr.0 - delta),
                    _ => unreachable!(),
                }
                eval.clone()
            };

            eval = select_best_evaluator(vec![w_org, w_pos, w_neg]);
        }

        dbg!(eval);
        println!();
    }
}
