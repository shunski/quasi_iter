use quasi_iter::prelude::*;

#[test]
fn merge() {
    assert_eq!( 
        Vec::<u32>::new().into_iter().merge(
            Vec::<u32>::new()
        ).next(), 
        None
    );

    let iter = 0u32..10;

    assert!( 
        iter.clone().into_iter().merge( 
            Vec::<u32>::new().into_iter()
        ).eq(iter.clone())
    );

    assert!( 
        Vec::<u32>::new().into_iter().merge(
            iter.clone()
        ).eq(iter.clone())
    );
    
    let iter1 = [1,3,4,5,8].into_iter();
    let iter2 = [1,2,6,7,8];

    assert!(
        iter1.merge(iter2).eq( [1,1,2,3,4,5,6,7,8,8] )
    );

}

#[test]
fn merge_dyn_test() {
    assert!(
        merge_dyn(
            (0u32..5).map(|i| Box::new((0..5).map(move |j| i+j*5 ).into_iter()) as Box<dyn Iterator<Item=u32>> )
        ).eq(0_u32..25)
    );
}