use quasi_iter::merge_sorted;

#[test]
fn trivial() {
    assert_eq!( 
        merge_sorted(
            Vec::<u32>::new().into_iter(), 
            Vec::<u32>::new().into_iter(),
        ).next(), 
        None
    );

    let iter = (0u32..10).into_iter();

    assert!( 
        merge_sorted(
            iter.clone(), 
            Vec::<u32>::new().into_iter(),
        ).eq(iter.clone())
    );

    assert!( 
        merge_sorted(
            Vec::<u32>::new().into_iter(),
            iter.clone(), 
        ).eq(iter.clone())
    );
}

