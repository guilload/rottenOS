use core::mem::size_of;
use core::prelude::*;


extern {
    fn gdt_load(pointer: *const GDT);
}


#[repr(packed)]
struct GDTDescriptor {
    limit_low: u16,  // the lower 16 bits of the limit
    base_low: u16,  // the lower 16 bits of the base
    base_middle: u8,  // the next 8 bits of the base
    access: u8,  // access flags, determine what ring this segment can be used in
    granularity: u8,
    base_high: u8
}

impl GDTDescriptor {

    fn new(base: u32, limit: uint, access: u8, granularity: u8) -> GDTDescriptor {
        GDTDescriptor {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: (base >> 16 & 0xFF) as u8,
            access: access,
            granularity: ((limit >> 16 & 0x0F) as u8) | granularity & 0xF0,
            base_high: (base >> 24 & 0xFF) as u8,
        }
    }

    fn flat(access: u8, granularity: u8) -> GDTDescriptor {
        GDTDescriptor::new(0, 0xFFFFFFFF, access, granularity)
    }

    fn null() -> GDTDescriptor {
        GDTDescriptor::new(0, 0, 0, 0)
    }
}


const GDT_SIZE: uint = 5;

type GDTable = [GDTDescriptor; GDT_SIZE];

static mut GDTABLE: GDTable = [
    GDTDescriptor {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    }; GDT_SIZE
];


#[repr(packed)]
struct GDT {
    limit: u16,
    base: *const GDTable,
}

impl GDT {

    fn new() -> GDT {
        unsafe {
            GDTABLE = [
                GDTDescriptor::null(),  // null segment
                GDTDescriptor::flat(0x9A, 0xCF),  // code segment
                GDTDescriptor::flat(0x92, 0xCF),  // data segment
                GDTDescriptor::flat(0xFA, 0xCF),  // user mode code segment
                GDTDescriptor::flat(0xF2, 0xCF),  // user mode data segment
            ];

            GDT {
                limit: (size_of::<GDTDescriptor>() * GDTABLE.len() - 1) as u16,
                base: &GDTABLE as *const GDTable,
            }
        }
    }

    fn load(&self) {
        unsafe {
            gdt_load(self as *const GDT);
        }
    }
}


pub fn init() {
    GDT::new().load();
}
