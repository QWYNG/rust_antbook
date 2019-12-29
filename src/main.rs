fn main() {
    println!("Hello, world!");
}

fn binary_search(arry: &Vec<usize>, x: usize) -> bool {
    if arry.len() <= 1 && arry.last().unwrap() != &x {
        return false
    }

    let mid = arry.len() / 2;
    let n = arry[mid];

    if  n == x {
        true
    } else if x > n  {
        binary_search(&arry[mid..].to_vec(), x)
    } else
    {
        binary_search(&arry[0..(mid - 1)].to_vec(), x)
    }
}

#[test]
fn binary_search_test() {
    let arry = vec![1, 2, 3, 4, 5];
    assert_eq!(binary_search(&arry, 1), true);
    assert_eq!(binary_search(&arry, 6), false);
}

