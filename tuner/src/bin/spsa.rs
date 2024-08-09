use bot::evaluator::*;
use itertools::izip;
use nova_tuner::simulate::select_best_evaluator_overrider;
use rand::Rng;

macro_rules! features {
    [$([$sign:tt, $delta_max:literal, $feat:ident]),* $(,)?] => {
        vec![$(
            (
                stringify!($feat),
                $delta_max,
                |mut eval: Evaluator, delta: i32| {
                    eval.$feat = match stringify!($sign) {
                        "+" => (eval.$feat + delta).max(0),
                        "-" => (eval.$feat + delta).min(0),
                        _ => unreachable!(),
                    };
                    eval
                },
                |eval: &Evaluator| eval.$feat,
            ),
        )*]
    };
}

fn main() {
    // Evaluator to tune
    let mut eval = BUILD_MIDGAME;

    let targets: Vec<(
        &str,
        i32,
        fn(Evaluator, i32) -> Evaluator,
        fn(&Evaluator) -> i32,
    )> = features![
        // [-, 10, bump],
        // [-, 10, dent],
        // [-, 10, dead_cells],
        // [+, 10, conn_2_v],
        // [+, 10, conn_2_h],
        // [-, 10, non_u_shape],
        // [-, 10, non_u_shape_sq],
        // [-, 10, frame],
        // [-, 10, frame_by_chain],
        // [-, 10, frame_by_chigiri],
        [-, 10, detected_need],
        [-, 10, detected_keys],
        [+, 15, detected_score_per_k],
    ];

    let initial_values: Vec<(&str, i32)> = targets
        .iter()
        .map(|target| (target.0, target.3(&eval)))
        .collect();
    let max_feat_len = initial_values.iter().map(|t| t.0.len()).max().unwrap();

    for i in 1..=100 {
        println!("> SPSA iteration {}", i);
        let before_values: Vec<i32> = targets.iter().map(|target| target.3(&eval)).collect();

        for (_, delta_max, tweaker, _) in &targets {
            let delta = rand::thread_rng().gen_range(2..=*delta_max);

            let w_org = eval.clone();
            let w_pos = tweaker(eval, delta);
            let w_neg = tweaker(eval, -delta);

            let o_org: EvaluatorOverrider = (eval.name, w_org);
            let o_pos: EvaluatorOverrider = (eval.name, w_pos);
            let o_neg: EvaluatorOverrider = (eval.name, w_neg);

            let best_o = select_best_evaluator_overrider(vec![o_org, o_pos, o_neg]);
            eval = best_o.1;
        }

        let after_values: Vec<i32> = targets.iter().map(|target| target.3(&eval)).collect();
        for ((feature_name, initial_value), before_value, after_value) in
            izip!(&initial_values, &before_values, &after_values)
        {
            println!(
                "- {:>max_feat_len$}: {:>4} ({:>4} against prev, {:>4} against init)",
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
