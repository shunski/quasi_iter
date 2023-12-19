use quasi_iter::{
    merge_sorted,
    merge_sorted_dyn
};

#[test]
fn merge() {
    assert_eq!( 
        merge_sorted(
            Vec::<u32>::new(), 
            Vec::<u32>::new(),
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
    
    let iter1 = [1,3,4,5,8];
    let iter2 = [1,2,6,7,8];

    assert!(
        merge_sorted(iter1, iter2)
            .eq( [1,1,2,3,4,5,6,7,8,8] )
    );

}

#[test]
fn merge_dyn() {
    assert!(
        merge_sorted_dyn(
            (0u32..5).map(|i| Box::new((0..5).map(move |j| i+j*5 ).into_iter()) as Box<dyn Iterator<Item=u32>> )
        ).eq(0_u32..25)
    );
}