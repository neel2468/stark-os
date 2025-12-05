#![no_std]
#![no_main]

use stark_os::{memory::mem, print, println};
use x86_64::{VirtAddr, structures::paging::{Page, Translate}};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    stark_os::hlt_loop();
}



fn kernel_main(boot_info: &'static BootInfo) -> ! {
    stark_os::init();
    println!("Stark OS v1.0{}", "!");

    let phy_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset
    ];

    let mut mapper = unsafe {
        mem::init(phy_mem_offset)
    };
    let mut frame_allocator = mem::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::new(0));
    mem::create_test_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0x_0b21_0b77_0b65_0b4e)
    };

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}",virt,phys);
    }
   

    stark_os::hlt_loop();
}