use std::os::raw::c_void;

use native_vtable::{vtable, vtable_impl};
use skyline::libc::c_char;

use crate::{
    eui::{
        box_cursor_node::BoxCursorNode,
        screen_manager::{ExtendedCloseState, ScreenMgr},
        Animator, ButtonBase, ButtonGroup, ControlBase, DrawState, LayoutEx, ScreenState,
    },
    sead::{
        container::{ListNode, OffsetList},
        heap::{Heap, IDisposer, VftIDisposer},
        prim::SafeString,
    },
};

#[vtable(path = eui::screen::VftScreen)]
pub unsafe trait VftScreen: VftIDisposer {
    unsafe fn check_derived_runtime_type_info(&self, type_info: *const c_void) -> *const Self;
    unsafe fn get_runtime_type_info(&self) -> *const c_void;
    unsafe fn is_enable_control(&self) -> bool;
    unsafe fn open(&mut self, target_state: i32) -> *mut Self;
    unsafe fn close(&mut self, target_state: i32);
    unsafe fn get_ui_controller(&self) -> *const c_void;
    unsafe fn adjust_box_cursor_node(&mut self);
    unsafe fn create_box_cursor_node(&mut self, heap: *mut Heap) -> *mut BoxCursorNode;

    unsafe fn replace_parts_layout_name(
        &self,
        name: *const c_char,
        parts: *mut c_void,
        layout: *mut LayoutEx,
    ) -> *const c_char;

    // REMINDER: +0x58 has afterBuildPane_, but skipping it because no idea what the signature is

    #[vtable(offset = 0x60)]
    unsafe fn set_animator_state(&mut self, state: u32, animator: *mut Animator);
    unsafe fn do_create_letter_anim_control(&mut self, heap: *mut Heap) -> *mut ControlBase;
    unsafe fn do_create_number_anim_control(&mut self, heap: *mut Heap) -> *mut ControlBase;

    // +0x78 postInitialize_, same reasoning as above

    #[vtable(offset = 0x80)]
    unsafe fn initialize(
        &mut self,
        manager: *mut ScreenMgr,
        heap: *mut Heap,
        arg_3: usize,
        name: *const c_char,
        screen_index: i32,
        draw_unit_id: i8,
        is_touch: bool,
    );

    unsafe fn update(&mut self);
    unsafe fn draw(&mut self, render_buffer_info: *const c_void);

    // +0x98

    #[vtable(offset = 0xa0)]
    unsafe fn process_animation_command(
        &mut self,
        animator: *mut Animator,
        command: u32,
        targets: *const c_char,
        target_count: u32,
    );

    unsafe fn get_layout_name(&self) -> *const c_char;
    unsafe fn get_layout_name_2(&self) -> *const c_char;
    unsafe fn get_layout_name_3(&self) -> *const c_char;
    unsafe fn is_play_parts_in_out(&self) -> bool;
    unsafe fn is_disallow_hit_lower_screen_on_button_hit(&self) -> bool;
    unsafe fn do_create_layout(&mut self, heap: *mut Heap) -> *mut LayoutEx;
    unsafe fn do_create_draw_info_ex(&mut self, heap: *mut Heap) -> *mut c_void;
    unsafe fn do_create_button_group(&mut self, heap: *mut Heap) -> *mut ButtonGroup;
    unsafe fn do_after_build_layout(&mut self, heap: *mut Heap);
    unsafe fn do_setup_draw_info(&mut self);
    unsafe fn do_create_ui_controller(&mut self, heap: *mut Heap) -> *mut c_void;
    unsafe fn do_create_resource_accessor(&mut self, heap: *mut Heap) -> *mut c_void;
    unsafe fn do_create_tag_processor(&mut self, heap: *mut Heap) -> *mut c_void;
    unsafe fn do_build_layout(&mut self, name: *const SafeString, resource_accessor: *mut c_void);

    unsafe fn do_build_layout_impl(
        &mut self,
        name: *const SafeString,
        resource_accessor: *mut c_void,
        control_creator: *mut c_void,
        text_box_initializer: *mut c_void,
    ) -> u32;

    unsafe fn do_load_resource(&mut self, heap: *mut Heap);

    unsafe fn do_create_slide_list_control(
        &mut self,
        heap: *mut Heap,
        pane: *mut c_void,
        layout: *mut LayoutEx,
    ) -> *mut ControlBase;

    unsafe fn do_initialize(&mut self, heap: *mut Heap);
    unsafe fn do_update(&mut self);
    unsafe fn update_button(&mut self);
    unsafe fn get_animation_step(&self) -> f32;
    unsafe fn do_draw(&mut self, render_buffer_info: *const c_void);
    unsafe fn do_open_start(&mut self);
    unsafe fn do_open_end(&mut self);
    unsafe fn do_close_start(&mut self);
    unsafe fn do_close_end(&mut self);
    unsafe fn do_button_on_start(&mut self, button: *mut ButtonBase);
    unsafe fn do_button_on_end(&mut self, button: *mut ButtonBase);
    unsafe fn do_button_off_start(&mut self, button: *mut ButtonBase);
    unsafe fn do_button_off_end(&mut self, button: *mut ButtonBase);
    unsafe fn do_button_down_start(&mut self, button: *mut ButtonBase);
    unsafe fn do_button_down_end(&mut self, button: *mut ButtonBase);
    unsafe fn do_button_cancel_start(&mut self, button: *mut ButtonBase);
    unsafe fn do_button_cancel_end(&mut self, button: *mut ButtonBase);

    // +0x1b8 getElinkSystem_?

    #[vtable(offset = 0x1c0)]
    unsafe fn create_asset_info_reader(&mut self, slink_user_instance: *mut c_void) -> *mut c_void;
    unsafe fn get_slink_property_count(&self) -> i32;
    unsafe fn set_slink_property_def(&mut self, slink_user_instance: *mut c_void);
    unsafe fn get_elink_property_count(&self) -> i32;
    unsafe fn set_elink_property_def(&mut self, elink_user_instance: *mut c_void);

    unsafe fn copy_controls(
        &mut self,
        heap: *mut Heap,
        source_layout: *mut LayoutEx,
        destination_layout: *mut LayoutEx,
    ) -> i32;

    unsafe fn is_linefeed_by_character_height(&self) -> bool;
    unsafe fn unk_0x1f8(&mut self);

    // +0x200

    #[vtable(offset = 0x208)]
    unsafe fn update_control(&mut self);
    unsafe fn open_start(&mut self, option: i32);
    unsafe fn is_open_end(&self) -> bool;
    unsafe fn open_end(&mut self);
    unsafe fn close_start(&mut self, option: i32);
    unsafe fn is_close_end(&self) -> bool;
    unsafe fn close_end(&mut self);
    unsafe fn is_force_glb_mtx_dirty(&self) -> bool;
    unsafe fn update_animator(&mut self);
    unsafe fn register_controller(&mut self);
    unsafe fn unregister_controller(&mut self);

    unsafe fn setup_pane_after_build(
        &mut self,
        pane: *mut c_void,
        layout: *mut LayoutEx,
        effect_link_pane_count: *mut u32,
    );

    unsafe fn do_initialize_layout(&mut self, layout: *mut LayoutEx);
    unsafe fn count_effect_link_pane(&mut self, pane: *mut c_void, count: *mut u32);
    unsafe fn create_effect_link_user(&mut self, heap: *mut Heap, pane_count: u32) -> *mut c_void;
    unsafe fn create_sound_link_2_user(&mut self, heap: *mut Heap) -> *mut c_void;
    unsafe fn invoke_sound_link_2_event(&mut self, event: *const c_char);

    unsafe fn invoke_sound_link_2_button_event(
        &mut self,
        button: *mut ButtonBase,
        event: *const c_char,
    ) -> bool;

    unsafe fn invoke_sound_link_2_anim_play_event(
        &mut self,
        animator: *mut Animator,
        event: *const c_char,
        playback_rate: f32,
        parameter: u32,
    );
}

const _: () = {
    assert!(core::mem::size_of::<VftScreenLayout>() == 0x2a0);
    assert!(VftScreenLayout::OFFSET_INITIALIZE == 0x80);
    assert!(VftScreenLayout::OFFSET_UPDATE == 0x88);
    assert!(VftScreenLayout::OFFSET_GET_ANIMATION_STEP == 0x148);
    assert!(VftScreenLayout::OFFSET_INVOKE_SOUND_LINK_2_ANIM_PLAY_EVENT == 0x298);
};

#[repr(C)]
pub struct Screen {
    pub base: IDisposer,
    pub screen_manager: *const ScreenMgr,
    pub layout: *const LayoutEx,
    pub button_group: *const ButtonGroup,
    pub control_list: ListNode,
    pub static_control_list: ListNode,

    // TODO: create struct
    pub ui_controller: *const c_void,

    // TODO: create struct
    pub draw_info: *const c_void,

    // TODO: create struct
    pub tag_processor: *const c_void,
    pub animator_list: ListNode,
    pub box_cursor_list: OffsetList<BoxCursorNode>,
    pub unk_98: OffsetList<()>,
    pub animated_parts_screen_list: OffsetList<()>,
    pub non_animated_parts_screen_list: OffsetList<()>,

    pub heap: *const Heap,
    pub index: i32,
    pub unk_ec: i32,
    pub name: *const c_char,
    pub active_box_cursor_node: *const BoxCursorNode,
    pub reserved_active_box_cursor_node: *const BoxCursorNode,
    pub elink_user: *const c_void,
    pub slink_user: *const c_void,
    pub camera_fov: f32,
    pub draw_unit_id: u8,
    pub draw_state: DrawState,
    pub screen_state: ScreenState,
    pub extended_close_state: ExtendedCloseState,
    pub is_initialized: bool,
    pub unk_121: u8,
    pub is_force_inactive: bool,
    pub is_no_operation_button_on_se_enabled: bool,
    pub input_flags: u32,
}

const _: () = assert!(core::mem::size_of::<Screen>() == 0x128);

#[vtable_impl]
unsafe impl VftScreen for Screen {}
