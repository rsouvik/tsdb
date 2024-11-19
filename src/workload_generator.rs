
/*
Generate workloads from a distribution etc.
 */
pub trait WLGenerator {
    fn nextWrite();
    fn nextRead();
}