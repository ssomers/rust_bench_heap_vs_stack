#![cfg(test)]
#![feature(test)]
#![feature(allocator_api)]
#![feature(maybe_uninit_extra)]
#![feature(new_uninit)]
extern crate test;
use std::alloc::{Allocator, Global, Layout};
use std::mem::MaybeUninit;
use std::ptr::{addr_of_mut, NonNull};

struct Node {
    parent: Option<NonNull<Node>>,
    parent_idx: MaybeUninit<u16>,
    len: u16,
}

impl Node {
    fn heap_new() -> Box<Self> {
        unsafe {
            let mut leaf = Box::new_uninit();
            Node::init(leaf.as_mut_ptr());
            leaf.assume_init()
        }
    }

    fn init(this: *mut Self) {
        unsafe {
            addr_of_mut!((*this).parent).write(None);
            //addr_of_mut!((*this).parent_idx).write(MaybeUninit::uninit());
            //addr_of_mut!((*this).parent_idx).write(MaybeUninit::new(0));
            addr_of_mut!((*this).len).write(0);
        }
    }

    fn stack_new() -> Box<Self> {
        Box::new(Node {
            parent: None,
            parent_idx: MaybeUninit::uninit(),
            len: 0,
        })
    }
}

fn grey_box(node1: Box<Node>, node2: Box<Node>) {
    unsafe {
        let node1 = NonNull::from(Box::leak(node1));
        let mut node2 = NonNull::from(Box::leak(node2));
        node2.as_mut().parent = Some(node1);
        node2.as_mut().parent_idx.write(42);
        test::black_box(node2);
        Global.deallocate(node2.cast(), Layout::new::<Node>());
        Global.deallocate(node1.cast(), Layout::new::<Node>());
    }
}

#[bench]
fn heap(b: &mut test::Bencher) {
    b.iter(|| grey_box(Node::heap_new(), Node::heap_new()))
}

#[bench]
fn stack(b: &mut test::Bencher) {
    b.iter(|| grey_box(Node::stack_new(), Node::stack_new()))
}
