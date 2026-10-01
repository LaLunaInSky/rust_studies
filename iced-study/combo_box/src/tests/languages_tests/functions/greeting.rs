use crate::languages::Language;

#[test]
fn greeting_in_danish() {
    let greeting_in_danish = Language::greeting(
        &Language::Danish
    );

    let result_expect = "Halloy!";

    assert_eq!(
        greeting_in_danish,
        result_expect
    );
}

#[test]
fn greeting_in_english() {
    let greeting_in_english = Language::greeting(
        &Language::English
    );

    let result_expect = "Hello!";

    assert_eq!(
        greeting_in_english,
        result_expect
    );
}

#[test]
fn greeting_in_french() {
    let greeting_in_french = Language::greeting(
        &Language::French
    );

    let result_expect = "Salut!";

    assert_eq!(
        greeting_in_french,
        result_expect
    );
}

#[test]
fn greeting_in_german() {
    let greeting_in_german = Language::greeting(
        &Language::German
    );

    let result_expect = "Hallo!";

    assert_eq!(
        greeting_in_german,
        result_expect
    );
}

#[test]
fn greeting_in_italian() {
    let greeting_in_italian = Language::greeting(
        &Language::Italian
    );

    let result_expect = "Ciao!";

    assert_eq!(
        greeting_in_italian,
        result_expect
    );
}

#[test]
fn greeting_in_japanese() {
    let greeting_in_japanese = Language::greeting(
        &Language::Japanese
    );

    let result_expect = "こんにちは!";

    assert_eq!(
        greeting_in_japanese,
        result_expect
    );
}

#[test]
fn greeting_in_portuguese() {
    let greeting_in_portuguese = Language::greeting(
        &Language::Portuguese
    );

    let result_expect = "Olá!";

    assert_eq!(
        greeting_in_portuguese,
        result_expect
    );
}

#[test]
fn greeting_in_spanish() {
    let greeting_in_spanish = Language::greeting(
        &Language::Spanish
    );

    let result_expect = "¡Hola!";

    assert_eq!(
        greeting_in_spanish,
        result_expect
    );
}

#[test]
fn greeting_in_other() {
    let greeting_in_other = Language::greeting(
        &Language::Other
    );

    let result_expect = "... hello?";

    assert_eq!(
        greeting_in_other,
        result_expect
    );
}