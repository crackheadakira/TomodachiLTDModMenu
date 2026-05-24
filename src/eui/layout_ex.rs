use skyline::nn::ui2d::Pane;
use std::ffi::{c_char, c_void};

use crate::{eui::Animator, sead::container::ListNode};

#[repr(C)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
pub struct Layout {
    pub vtable: *const c_void,
    pub anim_trans_list: ListNode,
    pub root_pane: *const Pane,
    pub group_container: *const c_void,
    pub layout_size: Size,
    pub name: *const c_char,
    pub res_ext_user_data_list: *const c_void,
    pub resource_accessor: *const c_void,
    pub parts_pane_list: ListNode,
    pub get_user_shader_information_from_user_data_callback: *const c_void,
}

#[repr(C)]
pub struct LayoutEx {
    pub base: Layout,
    pub in_animator: *mut Animator,
    pub out_animator: *mut Animator,
    pub loop_animator: *mut Animator,
    pub wait_animator: *mut Animator,
    pub screen: *const c_void,
    pub parent: *mut LayoutEx,
    pub flags: u8,
    pub anim_state: u8,
    pub pad_92: [u8; 6],
}

impl LayoutEx {
    pub fn get_in_animator<'a>(&self) -> Option<&'a mut Animator> {
        if !self.in_animator.is_null() {
            unsafe { Some(&mut *self.in_animator) }
        } else {
            None
        }
    }

    pub fn get_out_animator<'a>(&self) -> Option<&'a mut Animator> {
        if !self.out_animator.is_null() {
            unsafe { Some(&mut *self.out_animator) }
        } else {
            None
        }
    }

    pub fn get_loop_animator<'a>(&self) -> Option<&'a mut Animator> {
        if !self.loop_animator.is_null() {
            unsafe { Some(&mut *self.loop_animator) }
        } else {
            None
        }
    }

    pub fn get_wait_animator<'a>(&self) -> Option<&'a mut Animator> {
        if !self.wait_animator.is_null() {
            unsafe { Some(&mut *self.wait_animator) }
        } else {
            None
        }
    }
}

const _: () = assert!(core::mem::size_of::<Layout>() == 0x60);
const _: () = assert!(core::mem::size_of::<LayoutEx>() == 0x98);
