use crate::workdir::Workdir;

#[test]
fn tokenize() {
    let wrk = Workdir::new("tokenize");
    wrk.create(
        "data.csv",
        vec![
            svec!["n", "text"],
            svec!["1", "le chat mange"],
            svec!["2", "la souris"],
            svec!["3", ""],
        ],
    );
    let mut cmd = wrk.command("tokenize");
    cmd.arg("text").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["n", "token"],
        svec!["1", "le"],
        svec!["1", "chat"],
        svec!["1", "mange"],
        svec!["2", "la"],
        svec!["2", "souris"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn tokenize_column() {
    let wrk = Workdir::new("tokenize_column");
    wrk.create(
        "data.csv",
        vec![
            svec!["n", "text"],
            svec!["1", "le chat mange"],
            svec!["2", "la souris"],
            svec!["3", ""],
        ],
    );
    let mut cmd = wrk.command("tokenize");
    cmd.arg("text").args(["-c", "word"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["n", "word"],
        svec!["1", "le"],
        svec!["1", "chat"],
        svec!["1", "mange"],
        svec!["2", "la"],
        svec!["2", "souris"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn tokenize_token_type() {
    let wrk = Workdir::new("tokenize_token_type");
    wrk.create(
        "data.csv",
        vec![svec!["n", "text"], svec!["1", "1 chat mange 😎"]],
    );
    let mut cmd = wrk.command("tokenize");
    cmd.arg("text").args(["-T", "type"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["n", "token", "type"],
        svec!["1", "1", "number"],
        svec!["1", "chat", "word"],
        svec!["1", "mange", "word"],
        svec!["1", "😎", "emoji"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn tokenize_parallel() {
    let wrk = Workdir::new("tokenize_parallel");
    wrk.create(
        "data.csv",
        vec![
            svec!["n", "text"],
            svec!["1", "le chat mange"],
            svec!["2", "la souris"],
            svec!["3", ""],
        ],
    );
    let mut cmd = wrk.command("tokenize");
    cmd.arg("text").arg("-p").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["n", "token"],
        svec!["1", "le"],
        svec!["1", "chat"],
        svec!["1", "mange"],
        svec!["2", "la"],
        svec!["2", "souris"],
    ];
    assert_eq!(got, expected);
}
