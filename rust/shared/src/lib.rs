// check "docs/" for more info!

// Custom engine messages.
// WM_APP = 0x8000
//

pub mod message
{
    pub(crate) const WM_APP: u32 = 0x8000; // only for shared crate

    // and if you want anythink new always make a new value and doc it (if you remember hehe)
    pub const WM_ENGINE_TEST: u32 = WM_APP + 1; // 0x8000 + 1 = 0x8001
    pub const WM_ENGINE_EXIT: u32 = WM_APP + 2;
    pub const WM_ENGINE_BOOTUP_SUCCESS: u32 = WM_APP + 3;
    pub const WM_ENGINE_BOOTUP_FAILED: u32 = WM_APP + 4;
}