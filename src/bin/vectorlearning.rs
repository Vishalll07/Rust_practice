fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    // vec.push("string");
    println!("Vector is {:?}",vec);
}



// Adding Elements
// push(value: T): Adds an element to the end of the vector.
// insert(index: usize, value: T): Inserts an element at a specified index, shifting elements to the right.
// extend(iterable): Adds elements from an iterable (e.g., another vector or an array) to the end of the vector.

// 2. Removing Elements
// pop() -> Option<T>: Removes and returns the last element of the vector. Returns None if the vector is empty.
// remove(index: usize) -> T: Removes and returns the element at the specified index, shifting subsequent elements to the left.
// clear(): Removes all elements from the vector, leaving it empty.
// truncate(len: usize): Shortens the vector to the specified length. If len is greater than the current length, nothing happens.

// 3. Accessing Elements
// get(index: usize) -> Option<&T>: Returns a reference to the element at the given index, or None if the index is out of bounds.
// get_mut(index: usize) -> Option<&mut T>: Returns a mutable reference to the element at the given index.
// first() -> Option<&T>: Returns a reference to the first element, or None if the vector is empty.
// last() -> Option<&T>: Returns a reference to the last element, or None if the vector is empty.
// first_mut() -> Option<&mut T>: Returns a mutable reference to the first element.
// last_mut() -> Option<&mut T>: Returns a mutable reference to the last element.

// 4. Iterating Over Elements
// iter() -> Iter<T>: Returns an iterator over the vector’s elements.
// iter_mut() -> IterMut<T>: Returns a mutable iterator over the vector’s elements.
// into_iter() -> IntoIter<T>: Consumes the vector and returns an iterator that takes ownership of its elements.

// 5. Slicing
// as_slice() -> &[T]: Returns a slice of the entire vector as an immutable reference.
// as_mut_slice() -> &mut [T]: Returns a mutable slice of the entire vector.

// 6. Capacity and Resizing
// len() -> usize: Returns the number of elements in the vector.
// is_empty() -> bool: Checks if the vector is empty.
// capacity() -> usize: Returns the number of elements the vector can hold without reallocating.
// reserve(additional: usize): Reserves capacity for at least additional more elements.
// shrink_to_fit(): Shrinks the capacity of the vector as much as possible.
// resize(new_len: usize, value: T): Resizes the vector to new_len, adding value if the new length is greater than the current length.

// 7. Manipulating the Vector
// sort(): Sorts the elements in place using the default comparison.
// sort_by(|a, b| ...): Sorts the vector using a custom comparison function.
// reverse(): Reverses the order of elements in the vector.
// retain(|&x| ...): Retains only the elements specified by the predicate, removing others.

// 8. Cloning and Splitting
// clone() -> Vec<T>: Clones the vector, creating a new one with the same elements.
// split_off(at: usize) -> Vec<T>: Splits the vector into two at the given index, returning a new vector with elements from the index onward.

// 9. Miscellaneous
// drain(range) -> Drain<T>: Creates an iterator that removes elements in the specified range and returns them.
// concat() -> Vec<T>: Concatenates the elements of a vector of vectors into a single vector. (Vec<Vec<T>> becomes Vec<T>)
// dedup(): Removes consecutive duplicate elements.
// binary_search(&T) -> Result<usize, usize>: Searches for an element in a sorted vector using binary search.