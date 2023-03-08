mod chapters;

fn main() {
    chapter_one();
    chapter_two();
    chapter_three();
}

fn chapter_one() {
    println!("{:-^5} Chapter 1 {:-^5}", "-", "-");
    chapters::ch1::hello_world();
    chapters::ch1::comments();
    chapters::ch1::formatted_print();
    chapters::ch1::debug();
    chapters::ch1::debug_pretty();
    chapters::ch1::display_122_1();
    chapters::ch1::display_122_2();
    chapters::ch1::testcase_list();
    chapters::ch1::foratting_guide();
}

fn chapter_two() {
    println!("{:-^5} Chapter 2 {:-^5}", "-", "-");
    chapters::ch2::literals_and_operators();
    chapters::ch2::tuples_lesson();
    chapters::ch2::arrays_and_slices();
}

fn chapter_three() {
    
}
