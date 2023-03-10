mod chapters;

fn main() {
    chapter_one();
    chapter_two();
    chapter_three();
    chapter_four();
}

fn chapter_one() {
    use crate::chapters::ch1::*;
    println!("{:-^5} Chapter 1 {:-^5}", "-", "-");
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
    println!("{:-^5} Chapter 2 {:-^5}", "-", "-");
    literals_and_operators();
    tuples_lesson();
    arrays_and_slices();
}

fn chapter_three() {
    use crate::chapters::ch3::*;
    println!("{:-^5} Chapter 3 {:-^5}", "-", "-");
    structures();
    enum_nom_nom();
    enums_use();
    c_like();
    testcase_linked_list();
    constants();
}

fn chapter_four() {
    use crate::chapters::ch4::*;
    println!("{:-^5} Chapter 3 {:-^5}", "-", "-");

    mutability();
    scope_and_shadow();
}
