use crate::runtime::{*};

// (+ {a0 a1} b)
// --------------------- OP2-SUP-0
// dup b0 b1 = b
// {(+ a0 b0) (+ a1 b1)}
#[inline(always)]
pub fn apply(heap: &Heap, arit: &Arit, tid: usize, host: u64, term: Ptr, arg0: Ptr, arg1: Ptr) {
  inc_cost(heap, tid);
  let op20 = get_loc(term, 0);
  let op21 = get_loc(arg0, 0);
  let let0 = alloc(heap, tid, 3);
  let par0 = alloc(heap, tid, 2);
  link(heap, let0 + 2, arg1);
  link(heap, op20 + 1, Dp0(get_ext(arg0), let0));
  link(heap, op20 + 0, take_arg(heap, arg0, 0));
  link(heap, op21 + 0, take_arg(heap, arg0, 1));
  link(heap, op21 + 1, Dp1(get_ext(arg0), let0));
  link(heap, par0 + 0, Op2(get_ext(term), op20));
  link(heap, par0 + 1, Op2(get_ext(term), op21));
  let done = Sup(get_ext(arg0), par0);
  link(heap, host, done);
}
