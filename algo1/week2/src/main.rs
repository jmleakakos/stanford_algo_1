use std::fs::File;
use std::io::Read;
use std::usize;
 
static mut COMPARISON_COUNT : usize = 0;
static mut COMPARISON_COUNT_REAL : usize = 0;

fn get_data() -> Vec<i32>{
    let mut f = File::open("./quicksort.txt").unwrap();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let split = buffer.lines();
    let mut array = Vec::new();
    for s in split {
        match s.parse::<i32>() {
            Ok(num) => array.push(num),
            Err(err) => println!("ERROR: {}", err),
        }
    }
    return array
}

fn quicksort(array : &mut Vec<i32>, start : usize, end : usize) {
    //println!("START: {}, END: {}", start, end);
    if end-start == 0 {
        return
    } else {
        //println!("PIVOT ELEMENT: {}", array[start]);
        // When using the start as the pivot always, no need to do anything special

        // When using the end as the pivot always
        //array.swap(start, end);

        // When using the 'median-of-three'; find median of first, last, and middle of the array
        // use kth when 2k (even)
        // Divide by 2, then take the ceiling to get the middle
        //let middle_index = ((start + ((end - start)/2)) as f32).ceil() as usize;
        //let first = array[start];
        //let middle = array[middle_index];
        //let last = array[end];
        //let mut all_three = vec![(first, start), (middle, middle_index), (last, end)];
        //all_three.sort_by_key(|t| t.0);
        //let (_, median_index) = all_three[1];
        //array.swap(median_index, start);
        



        let pivot_index = partition(array, start, end);




        //println!("PIVOT INDEX: {}", pivot_index);

        let new_left_start;
        let new_left_end;
        let new_right_start;
        let new_right_end;

        if pivot_index == start {
            new_left_start = start;
            new_left_end = start;
            new_right_start = pivot_index + 1;
            new_right_end = end;
        } else if pivot_index == end {
            new_left_start = start;
            new_left_end = pivot_index - 1;
            new_right_start = end;
            new_right_end = end;
        } else {
            new_left_start = start;
            new_left_end = pivot_index - 1;
            new_right_start = pivot_index + 1;
            new_right_end = end;
        }

       // let new_left_start = start;
       // let new_left_end;
       // if pivot_index != start {
       //     new_left_end = pivot_index - 1;
       // } else {
       //     new_left_end = start
       // }
       // let new_right_start = pivot_index + 1;
       // let new_right_end = end;


        //println!("NEW_LEFT_START: {}, NEW_LEFT_END: {}", new_left_start, new_left_end);
        //println!("NEW_RIGHT_START: {}, NEW_RIGHT_END: {}", new_right_start, new_right_end);
        //println!("");

        quicksort(array, new_left_start, new_left_end);
        unsafe {
            let array_length = new_left_end - new_left_start + 1;
            COMPARISON_COUNT = COMPARISON_COUNT + array_length - 1;
        }

        quicksort(array, new_right_start, new_right_end);
        unsafe {
            let array_length = new_right_end - new_right_start + 1;
            COMPARISON_COUNT = COMPARISON_COUNT + array_length - 1;
        }
    }
}

fn partition(array : &mut Vec<i32>, l : usize, r : usize) -> usize {
    let p = array[l];
    let mut i = l + 1;
    for j in (l+1)..(r+1) {
        //println!("I: {}, J: {}, {:?}", i, j, &array[l..r]);
        unsafe {
            COMPARISON_COUNT_REAL = COMPARISON_COUNT_REAL + 1;
        }
        if array[j] < p {
            array.swap(j, i);
            //println!("AFTER SWAP: {:?}", &array[l..r]);
            i = i + 1;
        }
    }
    array.swap(l, i-1);
    //println!("AT END: {:?}", array);
    i - 1
}

fn main() {
    //let a = vec![11,9,10,3,14,13,8,2,1,5,4,7,6,12];
    let a = get_data();
    let length = a.len();
    let mut array = a;
    quicksort(&mut array, 0, length-1);
    for v in &array {
        println!("{}", v);
    }
    let mut pre_sorted = Vec::new();
    for x in 1..10001 {
        pre_sorted.push(x);
    }
    assert!(array == pre_sorted);
    unsafe {
        println!("CC: {}", COMPARISON_COUNT);
    }
    unsafe {
        println!("CCR: {}", COMPARISON_COUNT_REAL);
    }
}

