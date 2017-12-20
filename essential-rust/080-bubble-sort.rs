/* 

	Algorithms in Rust: Bubble Sort
	-------------------------------
	The bubble sort algorithm compare all elements in an array two by two and it
	swap their positions if the two elements are not sorted.
	It takes an array in and gives an array out.


 */


fn bubble_sort(vec_to_sort: &Vec<i32>) -> Vec<i32> {
	let mut result = vec_to_sort.clone();

/*	
	We define here a function "bubble_sort" that takes "vec_to_sort" which is a
	vector (a growable or dynamic array) of i32 and give us out another vector
	i32.

	Then we iterate and compare two by two all elements and swap the position of
	elements if needed.
 */

 	for i in 0..result.len() {
 		for y in 0..result.len() {
 			if result[i] < result[y] {
 				result.swap(i, y);
 			}
 		}
 	}

 	result
 }



/*
	But what if we want to pass and array of u32 or strings? We have to make the
	function generic and use closures (sort of anonymous functions)
 */

 fn bubble_sort_generic<T: std::clone::Clone, F>(vec_to_sort: &Vec<T>, compar: F) -> Vec<T>
 	where 
 		F : Fn(&T, &T) -> bool {
 			let mut result = vec_to_sort.clone();
 			for i in 0..result.len() {
 				for y in 0..result.len() {
 					if compar(&result[i], &result[y]) {
 						result.swap(i, y);
 					}
 				}
 			}
 	result
 }

/*
	We say here that the function is parameterized against T and F, where F is a
	function which will return a boolean.
	Now it's time to test our functions.

 */


#[test]
fn test_with_little_array() {
    let vec_to_sort = vec![10, 5, 8, 6, 7, 3];
    let result = vec![3, 5, 6, 7, 8, 10];
    let vec_sorted = bubble_sort_generic(&vec_to_sort, |x,y| x < y);
    assert_eq!(vec_sorted, result);
}

#[test]
fn test_with_big_array() {
    let vec_to_sort = vec![18, 2, 15, 20, 4, 9, 1, 3, 6, 12, 8, 5, 13, 7, 10, 14, 11, 16, 17, 19];
    let result = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let vec_sorted = bubble_sort_generic(&vec_to_sort, |x,y| x < y);
    assert_eq!(vec_sorted, result);
}



fn main() {
	
}