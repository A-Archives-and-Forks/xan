use std::cmp;

use crate::workdir::Workdir;

#[test]
fn sort_select() {
    let wrk = Workdir::new("sort_select");
    wrk.create("in.csv", vec![svec!["1", "b"], svec!["2", "a"]]);

    let mut cmd = wrk.command("sort");
    cmd.arg("--no-headers")
        .args(["--select", "1"])
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["2", "a"], svec!["1", "b"]];
    assert_eq!(got, expected);
}

#[test]
fn sort_numeric() {
    let wrk = Workdir::new("sort_numeric");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["LETTER", "b"],
            svec!["2", "c"],
            svec!["1", "d"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        //Non-numerics should be put first
        svec!["LETTER", "b"],
        svec!["1", "d"],
        svec!["2", "c"],
        svec!["10", "a"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_numeric_non_natural() {
    let wrk = Workdir::new("sort_numeric_non_natural");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["8.33", "a"],
            svec!["5", "b"],
            svec!["LETTER", "c"],
            svec!["7.4", "d"],
            svec!["3.33", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        //Non-numerics should be put first
        svec!["LETTER", "c"],
        svec!["3.33", "e"],
        svec!["5", "b"],
        svec!["7.4", "d"],
        svec!["8.33", "a"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_reverse() {
    let wrk = Workdir::new("sort_reverse");
    wrk.create(
        "in.csv",
        vec![svec!["R", "S"], svec!["1", "b"], svec!["2", "a"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-R").arg("--no-headers").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["R", "S"], svec!["2", "a"], svec!["1", "b"]];
    assert_eq!(got, expected);
}

#[test]
fn sort_uniq() {
    let wrk = Workdir::new("sort_unique");
    wrk.create(
        "in.csv",
        vec![
            svec!["number", "letter"],
            svec!["2", "c"],
            svec!["1", "a"],
            svec!["3", "f"],
            svec!["2", "b"],
            svec!["1", "d"],
            svec!["2", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-u").args(["-s", "number"]).arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number", "letter"],
        svec!["1", "a"],
        svec!["2", "c"],
        svec!["3", "f"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_count() {
    let wrk = Workdir::new("sort_count");
    wrk.create(
        "in.csv",
        vec![
            svec!["number", "letter"],
            svec!["2", "c"],
            svec!["1", "a"],
            svec!["3", "f"],
            svec!["2", "b"],
            svec!["1", "d"],
            svec!["2", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-u")
        .args(["-c", "duplicate_count"])
        .args(["-s", "number"])
        .arg("-N")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number", "letter", "duplicate_count"],
        svec!["1", "a", "2"],
        svec!["2", "c", "3"],
        svec!["3", "f", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_count_not_uniq() {
    let wrk = Workdir::new("sort_count");
    wrk.create(
        "in.csv",
        vec![
            svec!["number", "letter"],
            svec!["2", "c"],
            svec!["1", "a"],
            svec!["3", "f"],
            svec!["2", "b"],
            svec!["1", "d"],
            svec!["2", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.args(["-c", "duplicate_count"])
        .args(["-s", "number"])
        .arg("-N")
        .arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sort_count_empty() {
    let wrk = Workdir::new("sort_count");
    wrk.create("in.csv", vec![svec!["number", "letter"]]);

    let mut cmd = wrk.command("sort");
    cmd.arg("-u")
        .args(["-c", "duplicate_count"])
        .args(["-s", "number"])
        .arg("-N")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["number", "letter", "duplicate_count"]];
    assert_eq!(got, expected);
}

#[test]
fn sort_count_one_line() {
    let wrk = Workdir::new("sort_count");
    wrk.create("in.csv", vec![svec!["number", "letter"], svec!["2", "c"]]);

    let mut cmd = wrk.command("sort");
    cmd.arg("-u")
        .args(["-c", "duplicate_count"])
        .args(["-s", "number"])
        .arg("-N")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number", "letter", "duplicate_count"],
        svec!["2", "c", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_count_one_group() {
    let wrk = Workdir::new("sort_count");
    wrk.create(
        "in.csv",
        vec![
            svec!["number", "letter"],
            svec!["2", "c"],
            svec!["2", "b"],
            svec!["2", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-u")
        .args(["-c", "duplicate_count"])
        .args(["-s", "number"])
        .arg("-N")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number", "letter", "duplicate_count"],
        svec!["2", "c", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_unstable() {
    let wrk = Workdir::new("sort_unstable");
    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["2"], svec!["1"], svec!["3"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--unstable").arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["n"], svec!["1"], svec!["2"], svec!["3"]];
    assert_eq!(got, expected);
}

#[test]
fn sort_parallel() {
    let wrk = Workdir::new("sort_parallel");
    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["2"], svec!["1"], svec!["3"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--parallel").arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["n"], svec!["1"], svec!["2"], svec!["3"]];
    assert_eq!(got, expected);
}

#[test]
fn sort_parallel_unstable() {
    let wrk = Workdir::new("sort_parallel_unstable");
    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["2"], svec!["1"], svec!["3"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--parallel")
        .arg("--unstable")
        .arg("-N")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["n"], svec!["1"], svec!["2"], svec!["3"]];
    assert_eq!(got, expected);
}

#[test]
fn sort_check() {
    let wrk = Workdir::new("sort_check");
    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["2"], svec!["1"], svec!["3"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--check").arg("in.csv");
    wrk.assert_err(&mut cmd);

    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["1"], svec!["2"], svec!["3"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--check").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sort_check_reverse() {
    let wrk = Workdir::new("sort_check");
    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["2"], svec!["1"], svec!["3"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--check").arg("-R").arg("in.csv");
    wrk.assert_err(&mut cmd);

    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["3"], svec!["2"], svec!["1"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--check").arg("-R").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sort_check_numeric() {
    let wrk = Workdir::new("sort_check");
    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["1.0"], svec!["1"], svec!["-5"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--check").arg("in.csv");
    wrk.assert_err(&mut cmd);

    let mut cmd = wrk.command("sort");
    cmd.arg("--check").arg("-N").arg("in.csv");
    wrk.assert_err(&mut cmd);

    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["-5"], svec!["1"], svec!["1.0"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--check").arg("-N").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sort_check_numeric_reverse() {
    let wrk = Workdir::new("sort_check");
    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["-5"], svec!["1"], svec!["1.0"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--check").arg("-N").arg("-R").arg("in.csv");
    wrk.assert_err(&mut cmd);

    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["1"], svec!["1.0"], svec!["-5"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--check").arg("-N").arg("-R").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

/// Order `a` and `b` lexicographically using `Ord`
pub fn iter_cmp<A, L, R>(mut a: L, mut b: R) -> cmp::Ordering
where
    A: Ord,
    L: Iterator<Item = A>,
    R: Iterator<Item = A>,
{
    loop {
        match (a.next(), b.next()) {
            (None, None) => return cmp::Ordering::Equal,
            (None, _) => return cmp::Ordering::Less,
            (_, None) => return cmp::Ordering::Greater,
            (Some(x), Some(y)) => match x.cmp(&y) {
                cmp::Ordering::Equal => (),
                non_eq => return non_eq,
            },
        }
    }
}
