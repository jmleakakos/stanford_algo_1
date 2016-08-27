use std::fs::File;
use std::io::Read;
use std::usize;

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
        quicksort(array, new_right_start, new_right_end);
    }
}

fn partition(array : &mut Vec<i32>, l : usize, r : usize) -> usize {
    if l == 0 && r == 2 {
        //println!("{:?}", &array[l..r]);
    }
    let p = array[l];
    let mut i = l + 1;
    for j in (l+1)..(r+1) {
        //println!("I: {}, J: {}, {:?}", i, j, &array[l..r]);
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
    for v in array {
        println!("{}", v);
    }
}

