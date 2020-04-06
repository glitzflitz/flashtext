use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flashtext::keywordprocessor::KeywordProcessor;
use regex::Regex;
use std::collections::HashSet;

fn find_keywords() {
    let test_str = "Generating random paragraphs can be an excellent way for writers to get their creative flow going at the beginning of the day. The writer has no idea what topic the random paragraph will be about when it appears. This forces the writer to use creativity to complete one of three common writing challenges. The writer can use the paragraph as the first one of a short story and build upon it. A second option is to use the random paragraph somewhere in a short story they create. The third option is to have the random paragraph be the ending paragraph in a short story. No matter which of these challenges is undertaken, the writer is forced to use creativity to incorporate the paragraph into their writing.";

    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keyword("undertaken");
    let result = keywordprocessor.find_keywords(test_str);
    let mut expected = HashSet::new();
    expected.insert("undertaken".to_string());
    assert_eq!(result, expected);
}

fn use_regex() {
    let test_str = "Generating random paragraphs can be an excellent way for writers to get their creative flow going at the beginning of the day. The writer has no idea what topic the random paragraph will be about when it appears. This forces the writer to use creativity to complete one of three common writing challenges. The writer can use the paragraph as the first one of a short story and build upon it. A second option is to use the random paragraph somewhere in a short story they create. The third option is to have the random paragraph be the ending paragraph in a short story. No matter which of these challenges is undertaken, the writer is forced to use creativity to incorporate the paragraph into their writing.";

    let re = Regex::new("undertaken").unwrap();
    let m = re.find(test_str).unwrap();
    //println!("Yeeeeeeeeeeeeeeeehawwwwwwwwwwww");
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("find_keywords", |b| b.iter(|| find_keywords()));
    c.bench_function("use_regex", |b| b.iter((|| use_regex())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
