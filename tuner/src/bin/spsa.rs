use bot::evaluator::BUILD;
use itertools::izip;
use nova_tuner::simulate::select_best_evaluator;
use rand::Rng;

// NOTE: use BeamSearcher in Nova when running SPSA
fn main() {
    let mut eval = BUILD;

    let constrain_positive = |x: i32| x.max(0);
    let constrain_negative = |x: i32| x.min(0);

    #[rustfmt::skip]
    let ptrs = [
        // ("bump", &mut eval.bump as *mut i32, -1, 20),
        // ("dent", &mut eval.dent as *mut i32, -1, 20),
        ("dead_cells", &mut eval.dead_cells as *mut i32, -1, 10),
        // ("conn_2", &mut eval.conn_2 as *mut i32, 1, 10),
        // ("conn_3", &mut eval.conn_3 as *mut i32, 1, 10),
        // ("non_u_shape", &mut eval.non_u_shape as *mut i32, -1, 10),
        // ("non_u_shape_sq", &mut eval.non_u_shape_sq as *mut i32, -1, 10),
        // ("frame", &mut eval.frame as *mut i32, -1, 10),
        // ("frame_by_chain", &mut eval.frame_by_chain as *mut i32, -1, 10),
        // ("frame_by_chigiri", &mut eval.frame_by_chigiri as *mut i32, -1, 10),
        // ("detected_need", &mut eval.detected_need as *mut i32, -1, 10),
        // ("detected_keys", &mut eval.detected_keys as *mut i32, -1, 10),
        // ("detected_score_per_k", &mut eval.detected_score_per_k as *mut i32, 1, 20),
    ];

    let initial_values: Vec<(&str, i32)> =
        ptrs.iter().map(|ptr| (ptr.0, unsafe { *ptr.1 })).collect();

    for i in 1..=100 {
        println!("> SPSA iteration {}", i);
        let before_values: Vec<i32> = ptrs.iter().map(|ptr| unsafe { *ptr.1 }).collect();

        for ptr in ptrs {
            let delta = rand::thread_rng().gen_range(2..=ptr.3);

            let w_org = eval.clone();

            let w_pos = unsafe {
                match ptr.2 {
                    1 => *ptr.1 = constrain_positive(*ptr.1 + delta),
                    -1 => *ptr.1 = constrain_negative(*ptr.1 + delta),
                    _ => unreachable!(),
                }
                eval.clone()
            };

            eval = w_org;
            let w_neg = unsafe {
                match ptr.2 {
                    1 => *ptr.1 = constrain_positive(*ptr.1 - delta),
                    -1 => *ptr.1 = constrain_negative(*ptr.1 - delta),
                    _ => unreachable!(),
                }
                eval.clone()
            };

            eval = select_best_evaluator(vec![w_org, w_pos, w_neg]);
        }

        let after_values: Vec<i32> = ptrs.iter().map(|ptr| unsafe { *ptr.1 }).collect();
        for ((feature_name, initial_value), before_value, after_value) in
            izip!(&initial_values, &before_values, &after_values)
        {
            println!(
                "- {:>20}: {:>4} ({:>4} against prev, {:>4} against init)",
                feature_name,
                after_value,
                prettier_diff(after_value - before_value),
                prettier_diff(after_value - initial_value)
            );
        }
        println!();
    }
}

fn prettier_diff(diff: i32) -> String {
    if diff == 0 {
        return "+- 0".to_owned();
    }

    let prefix = if diff > 0 {
        "\x1b[48;5;161m+"
    } else {
        "\x1b[48;5;26m-"
    };
    format!("{}{:>3}\x1b[0m", prefix, diff.abs())
}
