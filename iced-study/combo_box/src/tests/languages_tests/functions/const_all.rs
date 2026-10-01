use crate::languages::Language;

#[test]
fn const_all_values() {
    let all_languages = Language::ALL.to_vec();

    let expected_result = vec!(
        Language::Danish,
        Language::English,
        Language::French,
        Language::German,
        Language::Italian,
        Language::Japanese,
        Language::Portuguese,
        Language::Spanish,
        Language::Other,
    );

    assert_eq!(
        all_languages,
        expected_result
    );
}