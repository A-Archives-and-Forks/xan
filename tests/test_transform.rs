use workdir::Workdir;

#[test]
fn transform() {
    let wrk = Workdir::new("transform");
    wrk.create(
        "data.csv",
        vec![svec!["a", "b"], svec!["1", "2"], svec!["2", "3"]],
    );
    let mut cmd = wrk.command("transform");
    cmd.arg("b").arg("add(a, b)").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["a", "b"], svec!["1", "3"], svec!["2", "5"]];
    assert_eq!(got, expected);
}

#[test]
fn transform_rename() {
    let wrk = Workdir::new("transform_rename");
    wrk.create(
        "data.csv",
        vec![svec!["a", "b"], svec!["1", "2"], svec!["2", "3"]],
    );
    let mut cmd = wrk.command("transform");
    cmd.arg("b")
        .arg("add(a, b)")
        .args(&["-r", "c"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["a", "c"], svec!["1", "3"], svec!["2", "5"]];
    assert_eq!(got, expected);
}

#[test]
fn transform_implicit() {
    let wrk = Workdir::new("transform_implicit");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["john", "davis"],
            svec!["mary", "sue"],
        ],
    );
    let mut cmd = wrk.command("transform");
    cmd.arg("surname")
        .arg("upper")
        .args(&["-r", "upper_surname"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "upper_surname"],
        svec!["john", "DAVIS"],
        svec!["mary", "SUE"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn transform_errors_panic() {
    let wrk = Workdir::new("transform_errors_panic");
    wrk.create(
        "data.csv",
        vec![svec!["a", "b"], svec!["1", "test"], svec!["2", "3"]],
    );
    let mut cmd = wrk.command("transform");
    cmd.arg("b").arg("add(a, b)").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn transform_errors_report() {
    let wrk = Workdir::new("transform_errors_report");
    wrk.create(
        "data.csv",
        vec![svec!["a", "b"], svec!["1", "test"], svec!["2", "3"]],
    );
    let mut cmd = wrk.command("transform");
    cmd.arg("b")
        .arg("add(a, b)")
        .args(&["-E", "report"])
        .args(&["--error-column", "error"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b", "error"],
        svec!["1", "", "error when calling function \"add\": cannot safely cast from type \"string\" to type \"number\""],
        svec!["2", "5", ""],
    ];
    assert_eq!(got, expected);
}

#[test]
fn transform_errors_ignore() {
    let wrk = Workdir::new("transform_errors_ignore");
    wrk.create(
        "data.csv",
        vec![svec!["a", "b"], svec!["1", "test"], svec!["2", "3"]],
    );
    let mut cmd = wrk.command("transform");
    cmd.arg("b")
        .arg("add(a, b)")
        .args(&["-E", "ignore"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["a", "b"], svec!["1", ""], svec!["2", "5"]];
    assert_eq!(got, expected);
}

#[test]
fn transform_errors_log() {
    let wrk = Workdir::new("transform_errors_log");
    wrk.create(
        "data.csv",
        vec![svec!["a", "b"], svec!["1", "test"], svec!["2", "3"]],
    );
    let mut cmd = wrk.command("transform");
    cmd.arg("b")
        .arg("add(a, b)")
        .args(&["-E", "log"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["a", "b",], svec!["1", "",], svec!["2", "5",]];
    assert_eq!(got, expected);
}
