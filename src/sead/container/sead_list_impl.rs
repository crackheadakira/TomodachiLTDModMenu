#[derive(Debug)]
#[repr(C)]
pub struct ListNode {
    pub prev: *mut ListNode,
    pub next: *mut ListNode,
}

const _: () = assert!(core::mem::size_of::<ListNode>() == 0x10);

impl ListNode {
    pub fn next(&self) -> *mut ListNode {
        self.next
    }

    pub fn prev(&self) -> *mut ListNode {
        self.prev
    }

    pub fn try_next(&self) -> Option<&ListNode> {
        if self.next.is_null() {
            None
        } else {
            unsafe { Some(&*self.next) }
        }
    }

    pub fn try_prev(&self) -> Option<&ListNode> {
        if self.prev.is_null() {
            None
        } else {
            unsafe { Some(&*self.prev) }
        }
    }

    pub fn is_linked(&self) -> bool {
        !self.next.is_null() || !self.prev.is_null()
    }
}

impl PartialEq for ListNode {
    fn eq(&self, other: &Self) -> bool {
        (self as *const ListNode) == (other as *const ListNode)
    }
}

#[repr(C, packed)]
pub struct ListImpl {
    pub start_end: ListNode,
    pub count: i32,
}

const _: () = assert!(core::mem::size_of::<ListImpl>() == 0x14);

// TODO impl wrappers:
// public: isEmpty(), size(), reverse(), shuffle(), shuffle_random(), checkLinks()
// protected: sort(), mergeSort(), pushBack(), pushFront(), popBack(), popFront(), insertBefore(), insertAfter(), erase()
// front(), back(), nth(), indexOf(), swap(), moveAfter(), moveBefore(), find(), uniq(), clear()
