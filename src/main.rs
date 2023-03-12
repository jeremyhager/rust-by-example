mod chapters;

fn main() {
    chapter_one();
    chapter_two();
    chapter_three();
    chapter_four();
    chapter_five();
    chapter_six();
    chater_seven();
}

fn chapter_marker(chapter_name: String) {
    println!("{:-^5} {} {:-^5}", "-",chapter_name, "-");
}

fn chapter_one() {
    use crate::chapters::ch1::*;
    chapter_marker(String::from("Chapter 1"));

    hello_world();
    comments();
    formatted_print();
    debug();
    debug_pretty();
    display_122_1();
    display_122_2();
    testcase_list();
    foratting_guide();
}

fn chapter_two() {
    use crate::chapters::ch2::*;
    chapter_marker(String::from("Chapter 2"));

    literals_and_operators();
    tuples_lesson();
    arrays_and_slices();
}

fn chapter_three() {
    use crate::chapters::ch3::*;
    chapter_marker(String::from("Chapter 3"));

    structures();
    enum_nom_nom();
    enums_use();
    c_like();
    testcase_linked_list();
    constants();
}

fn chapter_four() {
    use crate::chapters::ch4::*;
    chapter_marker(String::from("Chapter 4"));

    mutability();
    scope_and_shadow();
    declare_first();
    freezing();
}

fn chapter_five() {
    use crate::chapters::ch5::*;
    chapter_marker(String::from("Chapter 5"));

    casting();
    literals();
    inference();
    aliasing();
}

fn chapter_six() {
    use crate::chapters::ch6::*;
    chapter_marker(String::from("Chapter 6"));
    
    from_trait();
    into_trait();
    try_from_try_into();
    converting_to_string();
    parsing_a_string();
}

fn chater_seven() {
    use crate::chapters::ch7::*;
    chapter_marker(String::from("Chapter 7"));

    expressions();
}