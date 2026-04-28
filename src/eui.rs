// todo: fix this

#[derive(Debug)]
pub struct EuiController(u64);

impl EuiController {
    pub fn new(ptr: u64) -> Self {
        Self(ptr)
    }

    pub fn address(self) -> u64 {
        self.0
    }

    pub unsafe fn get_vtable_fn(&self, index: usize) -> u64 {
        let vtable = *(self.0 as *const u64);
        *(vtable as *const u64).add(index)
    }

    pub unsafe fn delete(&self) {
        let func: extern "C" fn(u64) = std::mem::transmute(self.get_vtable_fn(2));
        func(self.0);
    }

    pub unsafe fn close(&self, force: bool) {
        let func: extern "C" fn(u64, i32) = std::mem::transmute(self.get_vtable_fn(5));
        func(self.0, force as i32);
    }

    pub unsafe fn open(&self) {
        let func: extern "C" fn(u64) = std::mem::transmute(self.get_vtable_fn(6));
        func(self.0);
    }

    pub unsafe fn is_enable_control(&self) -> bool {
        let func: extern "C" fn(u64) -> bool = std::mem::transmute(self.get_vtable_fn(4));
        func(self.0)
    }

    pub unsafe fn update(&self, delta_time: f32) {
        let func: extern "C" fn(u64, f32) = std::mem::transmute(self.get_vtable_fn(17));
        func(self.0, delta_time);
    }

    pub unsafe fn clear_input(&self) {
        let func: extern "C" fn(u64) = std::mem::transmute(self.get_vtable_fn(20));
        func(self.0);
    }

    pub unsafe fn reset(&self) {
        let func: extern "C" fn(u64) = std::mem::transmute(self.get_vtable_fn(30));
        func(self.0);
    }

    pub unsafe fn get_ui_controller(&self) -> u64 {
        let func: extern "C" fn(u64) -> u64 = std::mem::transmute(self.get_vtable_fn(7));
        func(self.0)
    }

    pub unsafe fn get_layout_name(&self) -> *const u8 {
        let func: extern "C" fn(u64) -> *const u8 = std::mem::transmute(self.get_vtable_fn(15));
        func(self.0)
    }

    pub unsafe fn do_open_start(&self) {
        let func: extern "C" fn(u64) = std::mem::transmute(self.get_vtable_fn(43));
        func(self.0);
    }

    pub unsafe fn do_open_end(&self) {
        let func: extern "C" fn(u64) = std::mem::transmute(self.get_vtable_fn(44));
        func(self.0);
    }

    pub unsafe fn do_close_start(&self) {
        let func: extern "C" fn(u64) = std::mem::transmute(self.get_vtable_fn(45));
        func(self.0);
    }

    pub unsafe fn do_button_down_end(&self, btn_ptr: u64) {
        let func: extern "C" fn(u64, u64) = std::mem::transmute(self.get_vtable_fn(52));
        func(self.0, btn_ptr);
    }

    pub unsafe fn do_button_cancel(&self, btn_ptr: u64) {
        let func: extern "C" fn(u64, u64) = std::mem::transmute(self.get_vtable_fn(54));
        func(self.0, btn_ptr);
    }
}
