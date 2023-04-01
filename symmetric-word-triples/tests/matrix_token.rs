use symmetric_word_triples::parser::matrix::TknMatrix;

#[test]
fn get_matrix_inbound_unset() {
    let tkn_matrix = TknMatrix::new(3);
    assert_eq!(tkn_matrix.get(0, 0), None);
}

#[test]
fn get_matrix_inbound_set() {
    let mut tkn_matrix = TknMatrix::new(3);
    tkn_matrix.matrix.push(Tkn(0));
    assert_eq!(tkn_matrix.get(0, 0), Some(Tkn(0)));
}

#[test]
#[should_panic]
fn get_matrix_outbound_unset() {
    let tkn_matrix = TknMatrix::new(3);

    let outbound = tkn_matrix.get(3, 3);
}

#[test]
fn get_matrix_inbound_set_nextrow() {
    let mut tkn_matrix = TknMatrix::new(2);
    tkn_matrix.matrix.push(Tkn(0));
    tkn_matrix.matrix.push(Tkn(1));
    tkn_matrix.matrix.push(Tkn(2));

    assert_eq!(tkn_matrix.get(1, 0), Some(Tkn(2)));
}

#[test]
fn matrix_push_to_capacity() {
    let mut tkn_matrix = TknMatrix::new(2);
    tkn_matrix.matrix.push(Tkn(0));
    tkn_matrix.matrix.push(Tkn(1));
    tkn_matrix.matrix.push(Tkn(2));
    tkn_matrix.matrix.push(Tkn(3));

    assert_eq!(tkn_matrix.get(0, 0), Some(Tkn(0)));
    assert_eq!(tkn_matrix.get(0, 1), Some(Tkn(1)));
    assert_eq!(tkn_matrix.get(1, 0), Some(Tkn(2)));
    assert_eq!(tkn_matrix.get(1, 1), Some(Tkn(3)));
}

#[test]
fn matrix_get_push_over_capacity() {
    let mut tkn_matrix = TknMatrix::new(2);
    tkn_matrix.matrix.push(Tkn(0));
    tkn_matrix.matrix.push(Tkn(1));
    tkn_matrix.matrix.push(Tkn(2));
    tkn_matrix.matrix.push(Tkn(3));
    tkn_matrix.matrix.push(Tkn(4));

    assert_eq!(tkn_matrix.get(1, 0), Some(Tkn(2)));
}

#[test]
fn matrix_insert_and_get_inbound() {
    let mut tkn_matrix = TknMatrix::new(2);
    tkn_matrix.insert(0, 0, Tkn(0));
    tkn_matrix.insert(0, 1, Tkn(1));
    tkn_matrix.insert(1, 0, Tkn(2));
    tkn_matrix.insert(1, 1, Tkn(3));

    assert_eq!(tkn_matrix.get(0, 0), Some(Tkn(0)));
    assert_eq!(tkn_matrix.get(0, 1), Some(Tkn(1)));
    assert_eq!(tkn_matrix.get(1, 0), Some(Tkn(2)));
    assert_eq!(tkn_matrix.get(1, 1), Some(Tkn(3)));
}

#[test]
#[should_panic]
fn matrix_insert_and_get_outbound() {
    let mut tkn_matrix = TknMatrix::new(2);
    tkn_matrix.insert(0, 0, Tkn(0));
    tkn_matrix.insert(0, 1, Tkn(1));
    tkn_matrix.insert(1, 0, Tkn(2));
    tkn_matrix.insert(1, 1, Tkn(3));

    let outbound = tkn_matrix.get(2, 2);
}

#[test]
#[should_panic]
fn matrix_insert_outbound_and_get() {
    let mut tkn_matrix = TknMatrix::new(2);
    tkn_matrix.insert(2, 2, Tkn(0));

    assert_eq!(tkn_matrix.get(2, 2), None);
}

#[test]
fn matrix_get_row_inbound() {
    let mut tkn_matrix = TknMatrix::new(2);
    tkn_matrix.insert(0, 0, Tkn(0));
    tkn_matrix.insert(0, 1, Tkn(1));
    tkn_matrix.insert(1, 0, Tkn(2));
    tkn_matrix.insert(1, 1, Tkn(3));

    let row = tkn_matrix.get_row(1);
    // println!("{row:?}");
    assert_eq!((row[0], row[1]), (Tkn(2), Tkn(3)));
}

#[test]
#[should_panic]
fn matrix_get_row_outbound() {
    let mut tkn_matrix = TknMatrix::new(2);
    tkn_matrix.insert(0, 0, Tkn(0));
    tkn_matrix.insert(0, 1, Tkn(1));
    tkn_matrix.insert(1, 0, Tkn(2));
    tkn_matrix.insert(1, 1, Tkn(3));

    let outbound = tkn_matrix.get_row(2);
}
