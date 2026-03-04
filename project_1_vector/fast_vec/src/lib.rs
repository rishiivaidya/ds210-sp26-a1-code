use std::{fmt::{Display, Formatter}, ptr::{self, null_mut}};

use malloc::MALLOC;

pub struct FastVec<T> {
    ptr_to_data: *mut T,
    len: usize,
    capacity: usize,
}
impl<T> FastVec<T> {
    // Creating a new FastVec that is either empty or has capacity for some future elements.
    pub fn new() -> FastVec<T> {
        return FastVec::with_capacity(1);
    }
    pub fn with_capacity(capacity: usize) -> FastVec<T> {
        return FastVec {
            ptr_to_data: MALLOC.malloc(size_of::<T>() * capacity) as *mut T,
            len: 0,
            capacity: capacity,
        };
    }

    // Retrieve the FastVec's length and capacity
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    // Transforms an instance of SlowVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len);
        for i in 0..self.len {
            unsafe {
                let ptr = self.ptr_to_data.add(i);
                let element = ptr::read(ptr);
                v.push(element);
            }
        }
        MALLOC.free(self.ptr_to_data as *mut u8);
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
        return v;
    }

    // Transforms a vector to a SlowVec.
    pub fn from_vec(vec: Vec<T>) -> FastVec<T> {
        let mut fast_vec: FastVec<T> = FastVec::with_capacity(vec.len());
        for element in vec {
            unsafe {
                let ptr = fast_vec.ptr_to_data.add(fast_vec.len);
                ptr::write(ptr, element);
            }
            fast_vec.len = fast_vec.len + 1;
        }
        return fast_vec;
    }

    // Student 1 and Student 2 should implement this together
    // Use the project handout as a guide for this part!
    pub fn get(&self, i: usize) -> &T { 

        todo!("implement get!");
    }

    // Student 2 should implement this.
    pub fn push(&mut self, t: T) {
        if self.len == self.capacity {
            todo!("implement growing the vector by doubling the size!");
        } else {
            todo!("implement pushing t directly since the vector still has capacity!");
        }
    }

    // Student 1 should implement this.
    pub fn remove(&mut self, i: usize) {// Remove the element at index i and shift all elements after it to the left by one position
        for i in i..self.len-1 { //Iterate from index i to the second last index of the vector (self.len - 1). It shifts each element one position to the left
            unsafe { // unsafe block because performing raw pointer arithmetic and dereferencing raw pointers
                let src_ptr = self.ptr_to_data.add(i + 1); // This calculates the source pointer by adding i + 1 to the base pointer (self.ptr_to_data). This points to the element that is currently at index i + 1, which we want to move to index i.
                let dst_ptr = self.ptr_to_data.add(i); // This calculates the destination pointer by adding i to the base pointer (self.ptr_to_data). This points to the element at index i, which we want to overwrite with the element from index i + 1.
                ptr::copy(src_ptr, dst_ptr, 1); // This copies one element from the source pointer (src_ptr) to the destination pointer (dst_ptr). This effectively shifts the element at index i + 1 to index i, and so on for all subsequent elements until the end of the vector. After this loop, the last element of the vector will be duplicated, but we will decrease the length of the vector by one to effectively remove the last element.
            }
        }
        self.len = self.len - 1;
        MALLOC.free(unsafe { self.ptr_to_data.add(self.len) } as *mut u8); // This frees the memory allocated for the last element of the vector


    }
    // This appears correct but with further testing, you will notice it has a bug!
    // Student 1 and 2 should attempt to find and fix this bug.
    // Hint: check out case 2 in memory.rs, which you can run using
    //       cargo run --bin memory
    pub fn clear(&mut self) {
    // Drop all elements
    for i in 0..self.len { //Iterate through all elements in the vector
        unsafe {
            let ptr = self.ptr_to_data.add(i); // Calculate the pointer to the i-th element by adding i to the base pointer (self.ptr_to_data)
            ptr::drop_in_place(ptr); // This drops the element at the ptr in place
        }
    }

    // Keep the allocation
    self.len = 0;
}


// Destructor should clear the fast_vec to avoid leaking memory.
impl<T> Drop for FastVec<T> {
    fn drop(&mut self) {
        // Drop elements
        for i in 0..self.len {
            unsafe {
                let ptr = self.ptr_to_data.add(i);
                ptr::drop_in_place(ptr);
            }
        }

        // Free buffer
        if !self.ptr_to_data.is_null() { // Check if the pointer is not null before freeing
            MALLOC.free(self.ptr_to_data as *mut u8); // Free the memory allocated for the vector's data
            self.ptr_to_data = null_mut(); // Set the pointer to null after freeing 
        }
    }
}



// This allows printing FastVecs with println!.
impl<T: Display> Display for FastVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FastVec[")?;
        if self.len > 0 {
            for i in 0..self.len()-1 {
                write!(f, "{}, ", self.get(i))?;
            }
            write!(f, "{}", self.get(self.len - 1))?;
        }
        return write!(f, "]");
    }
}