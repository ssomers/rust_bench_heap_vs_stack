## What is this?

Benchmarks comparing the creation of (a stripped down mockup of) BTree nodes directly on the heap versus through the stack, for which a performance regression was reported.

I managed to get [Godbolt to produce some readable comparison](https://rust.godbolt.org/z/bh4Gxb). The stack implementation copies 128 zero bits at once with some fancy modern 128 bit instructions, while the heap implementation carefully copies 64 and 32 of those separately.

When the example sets `parent_idx` to some non-zero value, the stack implementation emits a

    mov qword ptr [rax + 8]

instead of a:

    mov dword ptr [rax + 8]

So I think the difference is that somehow the stack implementation realizes it has 64 bits of space to dump bits in, while the directly-on-the-heap implementation writes only 32 bits. Note that it doesn't limit to 16 bits, the size of `parent_id`, but overwrites the other 16 bit field len using the value that it knows it already has written earlier. As if the compiler thinks that writing 32 bits is better than writing 16 bits, but writing 64 bits (to an 8 byte aligned address) is only useful when it's to the stack.

There's no difference between heap and stack initialization (on my machine) if the initialized field is u8 or u32 instead of u16, while the size of the uninitialized field doesn't matter (unless it exceeds 32 bits, obviously).

## How to run this?

    rustup install nightly
    rustup run nightly cargo bench
