use rand::Rng;
use std::io::Write;

/// Produces an array of random integers.
/// Generates an array of random integers. Returns a vector if successful. Writes
/// errors to the provided writer.
///
/// Correct usage:
/// `cargo run 20 -5 5`
/// The command above requests 20 random integers between -5 and 5.
pub fn gen_random<W: Write>(input: &[String], err_log: &mut W) -> Option<Vec<i64>> {
    let mut min: i64 = 0;
    let mut max: i64 = 10;

    let parsed_count = match input.get(1) {
        None => {
            writeln!(err_log, "Please provide one or more arguments:").expect("failed to log err");
            writeln!(err_log, "rng (count):").expect("failed to log err");
            writeln!(err_log, "rng (count) (max):").expect("failed to log err");
            writeln!(err_log, "rng (count) (min) (max):").expect("failed to log err");
            writeln!(err_log, "default min = 0, max = 10").expect("failed to log err");
            writeln!(err_log, "ex: `rng 3 0 2` might yield `[0, 2, 1]`")
                .expect("failed to log err");
            return None;
        }
        Some(str_ct) => str_ct.parse::<usize>(),
    };

    let count = match parsed_count {
        Ok(ct) => ct,
        Err(e) => {
            writeln!(err_log, "`count` must be an unsigned integer: {:#?}", e)
                .expect("failed to log err");
            return None;
        }
    };

    if let Some(second) = input.get(2) {
        match second.parse::<i64>() {
            Ok(num) => {
                // Set this to max. We may downgrade it to min if a
                // third arg was passed.
                max = num;
            }
            Err(e) => {
                writeln!(
                    err_log,
                    "the second argument must be a signed integer: {:#?}",
                    e
                )
                .expect("failed to log err");
                return None;
            }
        };
    }

    if let Some(third) = input.get(3) {
        match third.parse::<i64>() {
            Ok(num) => {
                // The second value was actually the minmum. Reassign.
                min = max;
                max = num;
            }
            Err(e) => {
                writeln!(
                    err_log,
                    "the third argument must be a signed integer: {:#?}",
                    e
                )
                .expect("failed to log err");
                return None;
            }
        }
    }

    if min >= max {
        writeln!(
            err_log,
            "The input min exceeds max. Unable to process request."
        )
        .expect("failed to log err");
        return None;
    }

    let mut rng = rand::thread_rng();
    let mut output: Vec<i64> = Vec::with_capacity(count + 1);

    for _ in 0..count {
        output.push(rng.gen_range(min..=max));
    }

    Some(output)
}

#[cfg(test)]
mod unit_tests {
    use crate::gen_random;

    #[test]
    fn basic_rand() {
        let mut err_log: Vec<u8> = Vec::new();
        let request: Vec<String> = vec!["", "5"].into_iter().map(|el| el.to_string()).collect();
        let output =
            gen_random(&request, &mut err_log).expect("basic request should have succeeded");
        assert!(output.len() == 5, "should have received 5 numbers back");
        assert!(err_log.is_empty(), "no error should have been logged");
    }

    #[test]
    fn failing_rand() {
        let mut err_log: Vec<u8> = Vec::new();
        let mut prev_log_len = 0;

        let queries: Vec<Vec<&'static str>> = vec![
            vec!["", "-5"],
            vec!["", "5", "0.25"],
            vec!["", "5", "5", "2"],
            vec!["", "5", "5", "hello"],
        ];

        for str_query in queries.into_iter() {
            let query: Vec<String> = str_query.into_iter().map(|el| el.to_string()).collect();
            let output = gen_random(&query, &mut err_log);
            assert!(output.is_none(), "request should have failed");
            assert!(
                err_log.len() > prev_log_len,
                "error messages should have been logged"
            );
            prev_log_len = err_log.len();
        }
    }

    #[test]
    fn longer_reqs() {
        let mut err_log: Vec<u8> = Vec::new();
        let requested_range = -37..=50;
        let output: Vec<String> = vec!["", "100", "-37", "50"]
            .into_iter()
            .map(|el| el.to_string())
            .collect();
        let output = gen_random(&output, &mut err_log).expect("request should have succeeded");

        assert!(
            output.len() == 100,
            "output length should equal what was requested"
        );
        for el in output.iter() {
            assert!(
                requested_range.contains(el),
                "returned elements must be in the requested range"
            );
        }
    }
}
