mod error_histogram;
use error_histogram::hist;

fn main() {
   assert_eq!(hist("tpwaemuqxdmwqbqrjbeosjnejqorxdozsxnrgpgqkeihqwybzyymqeazfkyiucesxwutgszbenzvgxibxrlvmzihcb"), "u  3     ***\rw  4     ****\rx  6     ******\rz  6     ******");
}
