use std::ffi::c_void;

use crate::{eui::screen::Screen, sead::container::ListNode};

#[repr(C)]
pub struct BoxCursorNode {
    pub vtable: *const c_void,
    pub list_node: ListNode,

    // TODO: create struct
    pub anim_button: *const c_void,

    pub screen: *const Screen,
    pub linked_nodes: [*const BoxCursorNode; 4],
}

const _: () = assert!(core::mem::size_of::<BoxCursorNode>() == 0x48);
