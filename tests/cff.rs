//! The glyph_ids and cmap tables in some of the tests were generated by running Prince with a
//! small test document and getting [subset::subset] to print the values passed to it.

// Workaround rustfmt bug:
// https://github.com/rust-lang/rustfmt/issues/3794
#[path = "common.rs"]
mod common;

use std::fmt::Debug;

use itertools::Itertools;

use allsorts::binary::read::ReadScope;
use allsorts::binary::write::{WriteBinary, WriteBuffer};
use allsorts::cff::{CFFVariant, Charset, Dict, DictDefault, FontDict, Operand, CFF};
use allsorts::subset::subset;
use allsorts::tables::{OpenTypeData, OpenTypeFont};
use allsorts::tag;

use crate::common::read_fixture;

#[test]
fn test_read_write_cff_cid() {
    let buffer = read_fixture("tests/fonts/noto/NotoSansJP-Regular.otf");
    let scope = ReadScope::new(&buffer);

    let otf = scope.read::<OpenTypeFont>().unwrap();
    let ttf = match otf.data {
        OpenTypeData::Single(ttf) => ttf,
        OpenTypeData::Collection(_) => unreachable!(),
    };

    // Read
    let cff_table_data = ttf.read_table(&otf.scope, tag::CFF).unwrap().unwrap();
    let cff: CFF = cff_table_data
        .read::<CFF>()
        .expect("error parsing CFF table");

    // Write
    let mut buffer = WriteBuffer::new();
    CFF::write(&mut buffer, &cff).expect("error writing CFF table");

    // Re-read
    let cff2: CFF = ReadScope::new(buffer.bytes())
        .read::<CFF>()
        .expect("error parsing written CFF table");

    // Compare
    assert_eq!(cff2.header, cff.header);
    assert_eq!(cff2.name_index.count, cff.name_index.count);
    assert_eq!(cff2.string_index.len(), cff.string_index.len());
    assert_eq!(cff2.global_subr_index.count, cff.global_subr_index.count);
    assert_eq!(cff2.fonts.len(), cff.fonts.len());

    let actual = &cff2.fonts[0];
    let expected = &cff.fonts[0];
    compare_dicts(&actual.top_dict, &expected.top_dict);
    let actual_data = match &actual.data {
        CFFVariant::CID(cid) => cid,
        _ => panic!("expected CID data"),
    };
    let expected_data = match &expected.data {
        CFFVariant::CID(cid) => cid,
        _ => panic!("expected CID data"),
    };
    assert_eq!(
        actual.char_strings_index.len(),
        expected.char_strings_index.len()
    );
    assert_eq!(
        actual_data.font_dict_index.len(),
        expected_data.font_dict_index.len()
    );
    actual_data
        .font_dict_index
        .iter()
        .zip(expected_data.font_dict_index.iter())
        .for_each(|(left, right)| {
            let dict = ReadScope::new(left)
                .read::<FontDict>()
                .expect("unable to read actual FontDict");
            let dict2 = ReadScope::new(right)
                .read::<FontDict>()
                .expect("unable to read expected FontDict");
            compare_dicts(&dict, &dict2);
        });
    assert_eq!(
        actual_data.private_dicts.len(),
        expected_data.private_dicts.len()
    );
    actual_data
        .private_dicts
        .iter()
        .zip(expected_data.private_dicts.iter())
        .for_each(|(left, right)| compare_dicts(left, right));
    assert_eq!(
        actual_data
            .local_subr_indices
            .iter()
            .map(|maybe_index| maybe_index.as_ref().map(|index| index.count))
            .collect_vec(),
        expected_data
            .local_subr_indices
            .iter()
            .map(|maybe_index| maybe_index.as_ref().map(|index| index.count))
            .collect_vec(),
    );
    assert_eq!(actual_data.fd_select, expected_data.fd_select);
}

#[test]
fn test_read_write_cff_type_1() {
    let buffer = read_fixture("tests/fonts/opentype/Klei.otf");
    let scope = ReadScope::new(&buffer);

    let otf = scope.read::<OpenTypeFont>().unwrap();
    let ttf = match otf.data {
        OpenTypeData::Single(ttf) => ttf,
        OpenTypeData::Collection(_) => unreachable!(),
    };

    // Read
    let cff_table_data = ttf.read_table(&otf.scope, tag::CFF).unwrap().unwrap();
    let cff: CFF = cff_table_data
        .read::<CFF>()
        .expect("error parsing CFF table");

    // Write
    let mut buffer = WriteBuffer::new();
    CFF::write(&mut buffer, &cff).expect("error writing CFF table");

    // Re-read
    let cff2: CFF = ReadScope::new(buffer.bytes())
        .read::<CFF>()
        .expect("error parsing written CFF table");

    // Compare
    assert_eq!(cff2.header, cff.header);
    assert_eq!(cff2.name_index.count, cff.name_index.count);
    assert_eq!(cff2.string_index.len(), cff.string_index.len());
    assert_eq!(cff2.global_subr_index.count, cff.global_subr_index.count);
    assert_eq!(cff2.fonts.len(), cff.fonts.len());

    let actual = &cff2.fonts[0];
    let expected = &cff.fonts[0];
    compare_dicts(&actual.top_dict, &expected.top_dict);
    let actual_data = match &actual.data {
        CFFVariant::Type1(type1) => type1,
        _ => panic!("expected Type 1 data"),
    };
    let expected_data = match &expected.data {
        CFFVariant::Type1(type1) => type1,
        _ => panic!("expected Type 1 data"),
    };
    assert_eq!(
        actual.char_strings_index.len(),
        expected.char_strings_index.len()
    );
    compare_dicts(&actual_data.private_dict, &expected_data.private_dict);
    assert_eq!(
        actual_data
            .local_subr_index
            .as_ref()
            .map(|index| index.count),
        expected_data
            .local_subr_index
            .as_ref()
            .map(|index| index.count)
    );
}

#[test]
fn test_subset_cff_cid() {
    let buffer = read_fixture("tests/fonts/noto/NotoSansJP-Regular.otf");
    let opentype_file = ReadScope::new(&buffer).read::<OpenTypeFont<'_>>().unwrap();
    let mut glyph_ids = [
        0, 1, 2, 3, 4, 5, 6, 7, 14, 19, 20, 38, 39, 41, 42, 49, 50, 52, 66, 68, 69, 70, 72, 74, 77,
        78, 79, 80, 81, 83, 84, 85, 86, 88, 202, 281, 338, 345, 350, 370, 393, 396, 399, 405, 410,
        2522, 5221,
    ];
    assert!(subset(&opentype_file.table_provider(0).unwrap(), &mut glyph_ids,).is_ok());
}

#[test]
fn test_subset_cff_type1() {
    let buffer = read_fixture("tests/fonts/opentype/Klei.otf");
    let opentype_file = ReadScope::new(&buffer).read::<OpenTypeFont<'_>>().unwrap();
    let mut glyph_ids = [0, 1, 53, 66, 67, 70, 72, 73, 74, 79, 84, 85, 86];
    assert!(subset(&opentype_file.table_provider(0).unwrap(), &mut glyph_ids,).is_ok());
}

#[test]
fn test_subset_cff_type1_iso_adobe() {
    // This test checks that with suitable input the font is subset using the ISOAdobe charset
    // The selected glyphs ' !"#$%&' are in ISOAdobe order so the charset should be ISOAdobe.
    let buffer = read_fixture("tests/fonts/opentype/Klei.otf");
    let opentype_file = ReadScope::new(&buffer).read::<OpenTypeFont<'_>>().unwrap();
    let mut glyph_ids = [0, 1, 2, 3, 4, 5, 6, 7];
    let subset_buffer = subset(&opentype_file.table_provider(0).unwrap(), &mut glyph_ids).unwrap();
    let scope = ReadScope::new(&subset_buffer);

    let otf = scope.read::<OpenTypeFont>().unwrap();
    let ttf = match otf.data {
        OpenTypeData::Single(ttf) => ttf,
        OpenTypeData::Collection(_) => unreachable!(),
    };

    let cff_table_data = ttf.read_table(&otf.scope, tag::CFF).unwrap().unwrap();
    let cff: CFF = cff_table_data
        .read::<CFF>()
        .expect("error parsing CFF table");

    match cff.fonts[0].charset {
        Charset::ISOAdobe => {}
        _ => panic!("Expected ISOAdobe got something else"),
    }
}

// Compare two Dicts for equality but allow Operands that are Offsets to differ
fn compare_dicts<T: DictDefault + Debug>(actual: &Dict<T>, expected: &Dict<T>) {
    let same = actual.len() == expected.len()
        && actual
            .iter()
            .zip(expected.iter())
            .all(|((op, operands), (op2, operands2))| {
                op == op2
                    && operands.len() == operands2.len()
                    && operands.iter().zip(operands2.iter()).all(|pair| {
                        match pair {
                            // Allow offsets to differ in value
                            (Operand::Offset(_), Operand::Offset(_)) => true,
                            (left, right) => left == right,
                        }
                    })
            });

    if !same {
        panic!(
            r#"Dicts differ (ignoring Offset values): `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#,
            actual, expected
        );
    }
}
