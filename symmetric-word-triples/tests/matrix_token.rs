use symmetric_word_triples::parser::{matrix::TokenMatrix, token::Token};

#[test]
fn get_matrix_inbound_unset() {
    let tkn_matrix = TokenMatrix::new(3);
    assert_eq!(tkn_matrix.get(0, 0), None);
}

#[test]
#[should_panic]
fn get_matrix_outbound_unset() {
    let tkn_matrix = TokenMatrix::new(3);

    let outbound = tkn_matrix.get(3, 3);
}

#[test]
fn matrix_insert_and_get_inbound() {
    let mut tkn_matrix = TokenMatrix::new(2);
    tkn_matrix.insert(0, 0, Token(0));
    tkn_matrix.insert(0, 1, Token(1));
    tkn_matrix.insert(1, 0, Token(2));
    tkn_matrix.insert(1, 1, Token(3));

    assert_eq!(tkn_matrix.get(0, 0), Some(Token(0)));
    assert_eq!(tkn_matrix.get(0, 1), Some(Token(1)));
    assert_eq!(tkn_matrix.get(1, 0), Some(Token(2)));
    assert_eq!(tkn_matrix.get(1, 1), Some(Token(3)));
}

#[test]
#[should_panic]
fn matrix_insert_and_get_outbound() {
    let mut tkn_matrix = TokenMatrix::new(2);
    tkn_matrix.insert(0, 0, Token(0));
    tkn_matrix.insert(0, 1, Token(1));
    tkn_matrix.insert(1, 0, Token(2));
    tkn_matrix.insert(1, 1, Token(3));

    let outbound = tkn_matrix.get(2, 2);
}

#[test]
#[should_panic]
fn matrix_insert_outbound_and_get() {
    let mut tkn_matrix = TokenMatrix::new(2);
    tkn_matrix.insert(2, 2, Token(0));

    assert_eq!(tkn_matrix.get(2, 2), None);
}

#[test]
fn matrix_get_row_inbound() {
    let mut tkn_matrix = TokenMatrix::new(2);
    tkn_matrix.insert(0, 0, Token(0));
    tkn_matrix.insert(0, 1, Token(1));
    tkn_matrix.insert(1, 0, Token(2));
    tkn_matrix.insert(1, 1, Token(3));

    let row = tkn_matrix.get_row(1);
    // println!("{row:?}");
    assert_eq!((row[0], row[1]), (Token(2), Token(3)));
}

#[test]
#[should_panic]
fn matrix_get_row_outbound() {
    let mut tkn_matrix = TokenMatrix::new(2);
    tkn_matrix.insert(0, 0, Token(0));
    tkn_matrix.insert(0, 1, Token(1));
    tkn_matrix.insert(1, 0, Token(2));
    tkn_matrix.insert(1, 1, Token(3));

    let outbound = tkn_matrix.get_row(2);
}

#[test]
fn matrix_not_symmetric() {
    let mut tkn_matrix = TokenMatrix::new(2);
    tkn_matrix.insert(0, 0, Token(0));
    tkn_matrix.insert(0, 1, Token(1));
    tkn_matrix.insert(1, 0, Token(2));
    tkn_matrix.insert(1, 1, Token(3));

    assert_eq!(tkn_matrix.is_symmetric(), false);
}

#[test]
fn matrix_symmetric() {
    let mut tkn_matrix = TokenMatrix::new(2);
    tkn_matrix.insert(0, 0, Token(0));
    tkn_matrix.insert(0, 1, Token(1));
    tkn_matrix.insert(1, 0, Token(1));
    tkn_matrix.insert(1, 1, Token(0));

    assert_eq!(tkn_matrix.is_symmetric(), true);
}
