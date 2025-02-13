
pub(crate) fn insertion_sorting(arr:&mut Vec<i64>){

  let n=arr.len();
  for i in 1..n{
    let current_value=arr[i];
    let mut j=i as isize - 1;

    while  j >= 0 && arr[j as usize] >current_value{
     arr[(j + 1) as usize] = arr[j as usize];
       j -= 1;
    }
      arr[(j + 1) as usize] = current_value;
  }
}