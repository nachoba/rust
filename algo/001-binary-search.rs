/*
 

   Binary Search
   -------------
   Suppose you're searching for a person in the phone book. Their name starts  with
   K. You could start at the beginning and keep flipping pages until you get to the
   Ks. But you are more likely to  start at a pake in  the middle, because you know
   the Ks are going to be near the middle of the phone book.

   Or suppose you're searching for a word in a dictionary, and it starts with O. A-
   gain, you'll start near the middle.

   Now suppose you log on to Facebook. When you do, Facebook has to verify that you
   have an account on the site. So, it needs to search for your username in its da-
   tabase. Suppose your username is karlmageddon.  Facebook could start from the As
   and search for your name -but it makes  more sense for it to begin  somewhere in
   middle.
   
   This is a search problem. And all  these cases use the  same algorithm  to solve
   the problem: binary search.

   Binary search is an algorithm; its input is a sorted list of elements.  If an e-
   lement you are looking  for is in that list, binary  search returns the position 
   where it's located. Otherwise, binary search returns "null".


   The "binsearch" function takes a sorted array and an item. If the item is in the
   array, the function returns its position.

*/

fn binsearch<T: PartialOrd>(target: &T, collection: &[T]) -> Option<usize> {
    let mut lo: usize = 0;
    let mut hi: usize = collection.len();

    while lo < hi {
        let m: usize = (hi - lo) / 2 + lo;

        if *target == collection[m] {
            return Some(m);
        } else if *target < collection[m] {
            hi = m;
        } else {
            lo =  m + 1;
        }
    }
    return None;
}



#[test]
fn test_present() {
    // i32
    assert_eq!(Some(0), binsearch(&0_i32, &[0, 1, 2, 4, 8, 16]));
    assert_eq!(Some(1), binsearch(&1_i32, &[0, 1, 2, 4, 8, 16]));
    assert_eq!(Some(2), binsearch(&2_i32, &[0, 1, 2, 4, 8, 16]));
    assert_eq!(Some(3), binsearch(&4_i32, &[0, 1, 2, 4, 8, 16]));
    assert_eq!(Some(4), binsearch(&8_i32, &[0, 1, 2, 4, 8, 16]));
    assert_eq!(Some(5), binsearch(&16_i32, &[0, 1, 2, 4, 8, 16]));

    // char
    assert_eq!(Some(0), binsearch(&'a', &['a', 'b', 'c']));
    assert_eq!(Some(1), binsearch(&'b', &['a', 'b', 'c']));
    assert_eq!(Some(2), binsearch(&'c', &['a', 'b', 'c']));

    // vector
    assert_eq!(Some(0), binsearch(&0.0, &vec![0.0, 1.0, 2.0]));
    assert_eq!(Some(1), binsearch(&1.0, &vec![0.0, 1.0, 2.0]));
    assert_eq!(Some(2), binsearch(&2.0, &vec![0.0, 1.0, 2.0]));
}


#[test]
fn test_absent() {
    assert_eq!(None, binsearch(&0, &[]));
    assert_eq!(None, binsearch(&0, &[1]));
    assert_eq!(None, binsearch(&2, &[1]));
    assert_eq!(None, binsearch(&0, &[1, 3]));
    assert_eq!(None, binsearch(&2, &[1, 3]));
    assert_eq!(None, binsearch(&4, &[1, 3]));

    assert_eq!(None, binsearch(&3.14, &vec![]));
    assert_eq!(None, binsearch(&3.14, &vec![0.0, 1.0, 2.0]));
}



