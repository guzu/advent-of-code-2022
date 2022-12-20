///////////////////////////////////////////////////////////////////////
///////////  TESTS ////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////

use super::*;

#[test]
fn test_incr() {
    let list = vec![0, 2, 0, 0, 0];
    let refl = vec![0, 0, 0, 2, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
}

#[test]
fn test_decr() {
    let list = vec![0,  0, 0, -2, 0];
    let refl = vec![0, -2, 0,  0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
}

#[test]
fn test_rotate_simple_up() {
    let list = vec![0, 0, 2, 0, 0];
    let refl = vec![2, 0, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    let list = vec![4, 0, 0, 0, 0];
    let refl = vec![4, 0, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    let list = vec![0, 4, 0, 0, 0];
    let refl = vec![0, 4, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    let list = vec![0, 0, 0, 4, 0];
    let refl = vec![0, 0, 0, 4, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    let list = vec![0, 0, 0, 0, 4];
    let refl = vec![4, 0, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    let list = vec![4, 2, 1, 0, 0];
    let refl = vec![4, 0, 1, 2, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
}

#[test]
fn test_rotate_simple_down() {
    let list = vec![0, 0, -2, 0, 0];
    let refl = vec![0, 0, 0, 0, -2];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    let list = vec![-1, 0, 0,  0, 0];
    let refl = vec![ 0, 0, 0, -1, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    let list = vec![-2, 0, 0,  0, 0];
    let refl = vec![ 0, 0, -2, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    let list = vec![-3,  0, 0, 0, 0];
    let refl = vec![ 0, -3, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
}

#[test]
fn test_rotate_multiple_up() {
    let list = vec![4, 0, 0, 0, 0];
    let refl = vec![4, 0, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    let list = vec![8, 0, 0, 0, 0];
    let refl = vec![8, 0, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    // Simple incremental check
    let list = vec![9, 0, 0, 0, 0];
    let refl = vec![0, 9, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    let list = vec![10, 0, 0, 0, 0];
    let refl = vec![0, 0, 10, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    let list = vec![11, 0, 0, 0, 0];
    let refl = vec![0, 0, 0, 11, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    let list = vec![12, 0, 0, 0, 0];
    let refl = vec![12, 0, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    let list = vec![16, 0, 0, 0, 0];
    let refl = vec![16, 0, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
}

#[test]
fn test_rotate_multiple_down() {
    let list = vec![-4, 0, 0, 0, 0];
    let refl = vec![-4, 0, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    let list = vec![-5, 0, 0,  0, 0];
    let refl = vec![ 0, 0, 0, -5, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    let list = vec![-8, 0, 0, 0, 0];
    let refl = vec![-8, 0, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    // Simple incremental check
    let list = vec![-9, 0, 0,  0, 0];
    let refl = vec![ 0, 0, 0, -9, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    let list = vec![-10, 0,   0, 0, 0];
    let refl = vec![  0, 0, -10, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    let list = vec![-11,   0, 0, 0, 0];
    let refl = vec![  0, -11, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
    
    let list = vec![-16, 0, 0, 0, 0];
    let refl = vec![-16, 0, 0, 0, 0];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
}

#[test]
fn test_mix_example() {
    // Example input and  from day 20
    let list = vec![1, 2, -3, 3, -2, 0,  4];
    let refl = vec![1, 2, -3, 4,  0, 3, -2];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
}

#[test]
fn test_answer_example() {
    // Example input and  from day 20
    let list = vec![1, 2, -3, 3, -2, 0, 4];
    let (out, map) = mix(&list, 1);
    let res = answer(&list, &out, &map);
    assert_eq!(res, 3);
}

#[test]
fn test_test1() {
    // Equivalent to test1.txt
    let list = vec![3, 1, 0, 2, 1, 4, 5];
    let refl = vec![0, 1, 3, 4, 2, 5, 1];
    let (out, _) = mix(&list, 1);
    assert_eq!(out, refl);
}
