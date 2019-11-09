mod error_histogram;
use error_histogram::hist2;

fn main() {
   assert_eq!(hist2("tpwaemuqxdmwqbqrjbeosjnejqorxdozsxnrgpgqkeihqwybzyymqeazfkyiucesxwutgszbenzvgxibxrlvmzihcb"), "u  3     ***\rw  4     ****\rx  6     ******\rz  6     ******");
}
