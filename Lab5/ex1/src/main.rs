use ex1::mem_inspect;
use ex1::List1;
use ex1::List2;

fn main() {
    let mut l1 = List1::List::<i32>::new();
    l1.push(10);
    let mut l2 = List2::List::<i32>::new();
    l2.push(10);

    mem_inspect::dump_object(&l1);
    mem_inspect::dump_object(&l2);
}
