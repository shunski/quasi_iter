use quasi_iter::prelude::*;

#[test]
fn dedup() {
    let x = [1,2,2,3,4,4,5];
    let y = [1,2,3,4,5];

    assert!(
        x.into_iter().dedup().eq( y ),
    );

    assert!(
        (0..0).into_iter().dedup().next() == None,
    );

    let x = [1,1,1,1,1,1];
    assert!(
        x.into_iter().dedup().eq([1]),
    );
}