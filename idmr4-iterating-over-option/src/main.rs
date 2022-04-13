fn main() {
    // Extend a collection
    // let grade:Option<&str> = None;
    let grade = Some("A+");
    let mut grades = vec!["B-", "C+", "D"];

    // if let Some(grade) = grade {
    //     grades.push(grade);
    // }
    // instead of the above if, we can use extend, in this sitiuation.
    // because grade: Option implicitly implements iterator
    // option some variant is collected, none variant is omitted.
    grades.extend(grade);
    println!("{grades:?}");

    // Extend an iterator
    for grade in grades.iter().chain(grade.iter()) {
        println!("{grade}");
    }

    // Filter out none variants
    let grades = vec![Some("A+"), None, Some("B+"), None];
    let grades: Vec<&str> = grades.into_iter().flatten().collect();
    println!("{grades:?}");

    // Map and filter out none variants
    let grades = ["3.8", "B+", "4.0", "A", "2.7"];
    let grades: Vec<f32> = grades
                            .iter()
                            .filter_map(|s| s.parse().ok())
                            .collect();
    println!("{grades:?}");
}
