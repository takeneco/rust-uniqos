
#![cfg(test)]

use std::mem;

use super::chain;

struct FwData {
    listnode: chain::ForwardNode,
    num: usize,
}

impl FwData {
    fn new(n: usize) -> Self {
        Self {
            listnode: chain::ForwardNode::new(),
            num: n,
        }
    }
}

impl Default for FwData {
    fn default() -> Self {
        Self {
            listnode: chain::ForwardNode::new(),
            num: 0,
        }
    }
}

fn refnode_r<'a>(e: &'a FwData) -> &'a chain::ForwardNode {
    &e.listnode
}
fn refnode_box(e: &Box<FwData>) -> &chain::ForwardNode {
    &e.listnode
}

#[test]
fn single_forward_list<'a>() {
    //let mut list1 = chain::SingleForwardList::new(|e: &Box<FwData>| &e.listnode);
    //let mut list1 = chain::ForwardList::new(|e: &FwData| { &e.listnode });
    let mut list1: chain::SingleForwardList<FwData, Box<FwData>, fn(&Box<FwData>)->&chain::ForwardNode> = chain::SingleForwardList::new(refnode_box);

    list1.push_front(Box::new(FwData::new(0)));
    list1.push_front(Box::new(FwData::new(1)));
    list1.push_front(Box::new(FwData::new(2)));

    for (n, d) in list1.into_iter().enumerate() {
        assert_eq!(d.num, 2 - n);
    }

    assert_eq!(mem::size_of_val(&list1), mem::size_of_val(&list1.end),
        "size = {}", mem::size_of_val(&list1));
}

#[test]
fn forward_list() {
    //let mut list1 = chain::ForwardList::new(|e: &FwData| unsafe{&(*e).listnode} as *const chain::ForwardNode<FwData>);
    //let mut list1 = chain::ForwardList::new(|e: &FwData| &e.listnode);
    let mut list1 = chain::ForwardList::new(refnode_box);

    list1.push_front(Box::new(FwData::new(0)));
    list1.push_front(Box::new(FwData::new(1)));
    list1.push_front(Box::new(FwData::new(2)));

    for (n, d) in list1.into_iter().enumerate() {
        assert_eq!(d.num, 2 - n);
    }

    assert_eq!(mem::size_of_val(&list1), mem::size_of_val(&list1.end),
        "size = {}", mem::size_of_val(&list1));
}

