fn main() {
    let mut a=[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 115, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut d=a.len()/2;
    while d>0{
      for i in d..a.len(){
        let j=a[i];
        let mut r=i;
        while r>=d{
          if j<a[r-d]{
            a[r]=a[r-d];
          }
          else{
            break;
          }
          r=r-d;
        }
        a[r]=j;
      }
      d=d/2
    }
  println!("{:?}",a);
}
