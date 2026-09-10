//! Provides a touch interface and accompanying logic to allow the user to interact with scripts, cheats and settings.

use super::{
    gui::{self, ns_string, CGFloat, NSInteger, NSUInteger, CGPoint, CGRect, CGSize},
    language::{Message, MessageKey},
};
use objc::{class, msg_send, runtime::Object, sel};
use once_cell::{sync::OnceCell, unsync::Lazy};
use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
};

pub enum RowDetail {
    Info(Message),
    Warning(Message),
}

pub trait RowData {
    fn title(&self) -> Message;
    fn detail(&self) -> RowDetail;
    fn value(&self) -> Message;
    fn tint(&self) -> Option<(u8, u8, u8)>;

    /// Should return true if the rows in the menu should be reloaded.
    fn handle_tap(&mut self) -> bool;
}

pub struct TabData {
    pub name: Message,
    pub warning: Option<Message>,
    pub row_data: Vec<Box<dyn RowData>>,
}

struct Row {
    data: Box<dyn RowData>,
    frame: CGRect,
    detail_label: *mut Object,
    value_label: *mut Object,
    button: *mut Object,
}

// fixme: New states don't incorporate controller stuff.
struct TabState {
    selected: bool,
    scroll_y: CGFloat,
}

const MENU_BACKGROUND_ALPHA: CGFloat = 0.2;
const MENU_INACTIVE_ALPHA: CGFloat = 0.1;

struct TabButton {
    message: Message,
    view: *mut Object,
}

impl TabButton {
    /// Sets the tab button's title based on `self.message`.
    fn reload_title(&mut self) {
        let title_string_objc = ns_string(self.message.translate());
        let font = super::language::current().font_set().title_uifont();

        unsafe {
            let _: () = msg_send![self.view, setTitle: title_string_objc forState: 0 as NSUInteger];

            let label: *mut Object = msg_send![self.view, titleLabel];
            let _: () = msg_send![label, setFont: font];
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ButtonTag {
    row: i16,
    tab: i8,
    is_close: bool,
}

impl ButtonTag {
    fn new_tab(index: usize) -> ButtonTag {
        ButtonTag {
            tab: index as i8,
            row: -1,
            is_close: false,
        }
    }

    fn new_row(tab: usize, row: usize) -> ButtonTag {
        ButtonTag {
            tab: tab as i8,
            row: row as i16,
            is_close: false,
        }
    }

    fn new_close() -> ButtonTag {
        ButtonTag {
            tab: -1,
            row: -1,
            is_close: true,
        }
    }

    fn to_ns_integer(self) -> NSInteger {
        let is_close_u8 = if self.is_close { 1u32 } else { 0u32 };
        let packed = (self.row as u16 as u32)
            | ((self.tab as u8 as u32) << 16)
            | (is_close_u8 << 24);
        packed as NSInteger
    }

    fn from_ns_integer(val: NSInteger) -> ButtonTag {
        let raw = val as u32;
        ButtonTag {
            row: (raw & 0xffff) as i16,
            tab: ((raw >> 16) & 0xff) as i8,
            is_close: ((raw >> 24) & 0xff) != 0,
        }
    }
}

static MESSAGE_SENDER: OnceCell<Mutex<Sender<MenuMessage>>> = OnceCell::new();

#[derive(Debug)]
pub enum MenuMessage {
    Show,
    Hide,

    ReloadRows,
    RebuildTab(usize),

    SelectTab(usize),
    HitRow(usize, usize),
}

impl MenuMessage {
    /// Send the message using the default sender. Requires locking the sender mutex, so
    /// will block until the mutex becomes available.
    pub fn send(self) {
        if let Some(sender) = MESSAGE_SENDER.get() {
            self.send_with_sender(&sender.lock().unwrap());
        } else {
            log::error!(
                "cannot send {:?} because sender is not initialised yet",
                self
            );
        }
    }

    /// Directly send this message using the specified sender.
    pub fn send_with_sender(self, sender: &Sender<Self>) {
        if let Err(err) = sender.send(self) {
            log::error!("Failed to send {:?}", err.0);
        }
    }
}

impl Row {
    fn new(data: Box<dyn RowData>, frame: CGRect) -> Row {
        let language = super::language::current();
        let font_set = language.font_set();

        unsafe {
            let button: *mut Object = msg_send![class!(UIButton), alloc];
            let button: *mut Object = msg_send![button, initWithFrame: frame];
            let _: () = msg_send![button, setOpaque: false];
            let _: () = msg_send![button, setBackgroundColor: gui::colours::clear()];

            let label: *mut Object = msg_send![button, titleLabel];
            let subtitle_font = font_set.subtitle_uifont();
            let _: () = msg_send![label, setFont: subtitle_font];

            let value_frame = CGRect::new(
                frame.size.width * (0.05 as CGFloat),
                0.0,
                frame.size.width * (0.9 as CGFloat),
                frame.size.height * (0.6 as CGFloat),
            )
            .rounded();

            let value_label: *mut Object = msg_send![class!(UILabel), alloc];
            let value_label: *mut Object = msg_send![value_label, initWithFrame: value_frame];
            let _: () = msg_send![value_label, setFont: subtitle_font];
            let _: () = msg_send![value_label, setOpaque: false];
            let _: () = msg_send![value_label, setBackgroundColor: gui::colours::clear()];

            let detail_frame = CGRect::new(
                frame.size.width * (0.05 as CGFloat),
                // 0.5 to move the detail up towards the title. This makes it more obvious that the
                //  detail goes with the title, and makes the rows easier to read.
                frame.size.height * (0.5 as CGFloat),
                frame.size.width * (0.9 as CGFloat),
                frame.size.height * (0.4 as CGFloat),
            )
            .rounded();

            let detail_label: *mut Object = msg_send![class!(UILabel), alloc];
            let detail_label: *mut Object = msg_send![detail_label, initWithFrame: detail_frame];

            let font = font_set.text_uifont();
            let _: () = msg_send![detail_label, setFont: font];
            let _: () = msg_send![detail_label, setAdjustsFontSizeToFitWidth: true];
            let _: () = msg_send![detail_label, setOpaque: false];
            let _: () = msg_send![detail_label, setBackgroundColor: gui::colours::clear()];

            let mut row = Row {
                data,
                frame,
                detail_label,
                value_label,
                button,
            };

            row.load();
            row
        }
    }

    fn load(&mut self) {
        self.align_text();

        let (detail_message, foreground_colour, background_colour) = match self.data.detail() {
            RowDetail::Info(s) => (
                s,
                gui::colours::white_with_alpha(1. as CGFloat, 0.95 as CGFloat),
                gui::colours::white_with_alpha(0. as CGFloat, 0. as CGFloat),
            ),
            RowDetail::Warning(s) => (
                s,
                gui::colours::get(gui::colours::ORANGE, 0.95 as CGFloat),
                gui::colours::get(gui::colours::ORANGE, MENU_BACKGROUND_ALPHA),
            ),
        };

        let (background_colour, value_colour) = if let Some(tint) = self.data.tint() {
            (
                gui::colours::get(tint, MENU_BACKGROUND_ALPHA),
                gui::colours::get(tint, 0.95 as CGFloat),
            )
        } else {
            (background_colour, gui::colours::white_with_alpha(1. as CGFloat, 0.95 as CGFloat))
        };

        let font_set = super::language::current().font_set();

        let title_str = self.data.title().translate();
        let value_str = self.data.value().translate();
        let detail_str = detail_message.translate();

        unsafe {
            let _: () = msg_send![self.button, setBackgroundColor: background_colour];
            let _: () = msg_send![self.button, setTitle: ns_string(title_str) forState: 0 as NSUInteger];
            let _: () = msg_send![self.button, setTitleColor: foreground_colour forState: 0 as NSUInteger];

            let _: () = msg_send![self.value_label, setText: ns_string(value_str)];
            let _: () = msg_send![self.value_label, setTextColor: value_colour];

            let _: () = msg_send![self.detail_label, setText: ns_string(detail_str)];
            let _: () = msg_send![self.detail_label, setTextColor: foreground_colour];

            let subtitle_font = font_set.subtitle_uifont();

            let btn_label: *mut Object = msg_send![self.button, titleLabel];
            let _: () = msg_send![btn_label, setFont: subtitle_font];
            let _: () = msg_send![self.value_label, setFont: subtitle_font];

            let _: () = msg_send![self.detail_label, setFont: font_set.text_uifont()];
        }
    }

    /// Aligns the row's text and adjusts it to match the language's direction.
    fn align_text(&mut self) {
        let language = super::language::current();

        let is_rtl = language.is_rtl();

        let button_alignment = if is_rtl {
            // Right
            2 as NSInteger
        } else {
            // Left
            1 as NSInteger
        };

        let value_alignment = if is_rtl {
            // Left
            0 as NSInteger
        } else {
            // Right
            2 as NSInteger
        };

        let desc_alignment = if is_rtl {
            // Right
            2 as NSInteger
        } else {
            // Left
            0 as NSInteger
        };

        let button_frame = self.frame;

        let button_edge_insets = if is_rtl {
            gui::UIEdgeInsets::new(
                0.,
                0.,
                button_frame.size.height * (0.4 as CGFloat),
                button_frame.size.width * (0.05 as CGFloat),
            )
        } else {
            gui::UIEdgeInsets::new(
                0.,
                button_frame.size.width * (0.05 as CGFloat),
                button_frame.size.height * (0.4 as CGFloat),
                0.,
            )
        };

        unsafe {
            let _: () = msg_send![self.button, setContentHorizontalAlignment: button_alignment];
            let _: () = msg_send![self.value_label, setTextAlignment: value_alignment];
            let _: () = msg_send![self.detail_label, setTextAlignment: desc_alignment];
            let _: () = msg_send![self.button, setTitleEdgeInsets: button_edge_insets];
        }
    }

    fn hit(&mut self) {
        if self.data.handle_tap() {
            MenuMessage::ReloadRows.send();
        }
    }
}

// Previously, we used multipliers for all of the element sizes. This produced good results on
//  smaller devices, but on iPads, many things were too big and the menu as a whole looked strange.
// Now we just hardcode the same values for all displays.

const ROW_HEIGHT: CGFloat = 50.;

const TAB_BUTTON_HEIGHT: CGFloat = 50.;

const CLOSE_BUTTON_HEIGHT: CGFloat = 35.;
const CLOSE_BTN_FONT_SIZE: CGFloat = 20.;

// Some elements are still proportional to others. The height of the warning label is proportional
//  to the height of the tab view as a whole.
const WARNING_HEIGHT_FRAC: CGFloat = 0.1;

struct Tab {
    name: Message,
    scroll_frame: CGRect,
    scroll_view: *mut Object,
    warning_message: Option<Message>,
    warning_label: Option<*mut Object>,
    rows: Vec<Row>,
}

impl Tab {
    fn new(data: TabData, tab_frame: CGRect, state: TabState) -> Tab {
        let language = super::language::current();

        let scroll_frame = if data.warning.is_some() {
            // Make the scroll view slightly shorter so we can fit the warning above it.
            CGRect::new(
                tab_frame.origin.x,
                tab_frame.origin.y + tab_frame.size.height * WARNING_HEIGHT_FRAC,
                tab_frame.size.width,
                tab_frame.size.height * ((1. as CGFloat) - WARNING_HEIGHT_FRAC),
            )
        } else {
            tab_frame
        };

        let scroll_view: *mut Object = unsafe {
            let scroll_view: *mut Object = msg_send![class!(UIScrollView), alloc];
            msg_send![scroll_view, initWithFrame: scroll_frame]
        };

        let row_width = scroll_frame.size.width;

        let make_row = |(index, data)| {
            Row::new(
                data,
                CGRect::new(0., ROW_HEIGHT * (index as CGFloat), row_width, ROW_HEIGHT),
            )
        };

        // Move all the RowData elements into Row structures.
        let rows: Vec<Row> = data
            .row_data
            .into_iter()
            .enumerate()
            .map(make_row)
            .collect();

        for row in &rows {
            unsafe {
                let _: () = msg_send![row.button, addSubview: row.value_label];
                let _: () = msg_send![row.button, addSubview: row.detail_label];
                let _: () = msg_send![scroll_view, addSubview: row.button];
            }
        }

        let content_size = CGSize {
            width: scroll_frame.size.width,
            height: ROW_HEIGHT * (rows.len() as CGFloat),
        };

        unsafe {
            let _: () = msg_send![scroll_view, setContentSize: content_size];

            let content_offset = CGPoint {
                x: 0.,
                y: state.scroll_y,
            };

            let _: () = msg_send![scroll_view, setContentOffset: content_offset animated: false];

            let background = gui::colours::white_with_alpha(0. as CGFloat, MENU_BACKGROUND_ALPHA);
            let _: () = msg_send![scroll_view, setBackgroundColor: background];
        }

        let warning_label = data.warning.as_ref().map(|warning| unsafe {
            let warning = warning.clone().translate();

            let warning_frame = CGRect::new(
                0.,
                tab_frame.origin.y,
                tab_frame.size.width,
                tab_frame.size.height * WARNING_HEIGHT_FRAC,
            );

            let label: *mut Object = msg_send![class!(UILabel), alloc];
            let label: *mut Object = msg_send![label, initWithFrame: warning_frame];

            let colour = gui::colours::get(gui::colours::ORANGE, 1. as CGFloat);
            let font = language.font_set().small_uifont();
            let _: () = msg_send![label, setTextColor: colour];
            let _: () = msg_send![label, setFont: font];
            let _: () = msg_send![label, setText: ns_string(warning)];
            let _: () = msg_send![label, setTextAlignment: 1 as NSInteger];
            let _: () = msg_send![label, setAdjustsFontSizeToFitWidth: true];
            let _: () = msg_send![label, setNumberOfLines: 0 as NSInteger];

            let colour = gui::colours::get((0, 0, 0), MENU_BACKGROUND_ALPHA);
            let _: () = msg_send![label, setBackgroundColor: colour];

            label
        });

        let mut tab = Tab {
            warning_message: data.warning,
            name: data.name,
            scroll_frame,
            scroll_view,
            warning_label,
            rows,
        };

        tab.set_selected(state.selected);
        tab
    }

    fn set_selected(&mut self, selected: bool) {
        unsafe {
            let _: () = msg_send![self.scroll_view, setHidden: !selected];

            if let Some(label) = self.warning_label {
                let _: () = msg_send![label, setHidden: !selected];
            }
        }
    }

    fn reload_warning(&mut self) {
        if let Some(warning) = self.warning_message.as_ref() {
            let label = self.warning_label.unwrap();

            unsafe {
                let _: () = msg_send![label, setText: ns_string(warning.translate())];
            }
        }
    }

    fn reload_fonts(&mut self) {
        let font_set = super::language::current().font_set();

        if let Some(warning_label) = self.warning_label {
            unsafe {
                let _: () = msg_send![warning_label, setFont: font_set.small_uifont()];
            }
        }
    }

    fn rebuild_rows(&mut self, data: TabData, tab_index: usize) {
        unsafe {
            for row in &self.rows {
                let _: () = msg_send![row.button, removeFromSuperview];
            }
        }

        let row_width = self.scroll_frame.size.width;

        let make_row = |(index, data)| {
            Row::new(
                data,
                CGRect::new(0., ROW_HEIGHT * (index as CGFloat), row_width, ROW_HEIGHT),
            )
        };

        self.rows = data
            .row_data
            .into_iter()
            .enumerate()
            .map(make_row)
            .collect();

        for (row_index, row) in self.rows.iter().enumerate() {
            unsafe {
                let _: () = msg_send![row.button, addSubview: row.value_label];
                let _: () = msg_send![row.button, addSubview: row.detail_label];
                let _: () = msg_send![self.scroll_view, addSubview: row.button];
            }
            add_button_handler(row.button, ButtonTag::new_row(tab_index, row_index));
        }

        let content_size = CGSize {
            width: row_width,
            height: ROW_HEIGHT * (self.rows.len() as CGFloat),
        };

        unsafe {
            let _: () = msg_send![self.scroll_view, setContentSize: content_size];

            let content_offset = CGPoint { x: 0., y: 0. };
            let _: () = msg_send![self.scroll_view, setContentOffset: content_offset animated: false];
        }
    }
}

impl TabButton {
    fn new(title: Message, index: usize, width: CGFloat) -> TabButton {
        let view = unsafe {
            let btn: *mut Object = msg_send![class!(UIButton), alloc];

            let frame = CGRect::new(width * (index as CGFloat), 0., width, TAB_BUTTON_HEIGHT);
            let btn: *mut Object = msg_send![btn, initWithFrame: frame];

            let _: () = msg_send![btn, setTitle: ns_string(title.translate()) forState: 0 as NSUInteger];

            let label: *mut Object = msg_send![btn, titleLabel];

            let font = super::language::current().font_set().title_uifont();

            let _: () = msg_send![label, setFont: font];
            let _: () = msg_send![label, setAdjustsFontSizeToFitWidth: true];

            add_button_handler(btn, ButtonTag::new_tab(index));

            btn
        };

        TabButton {
            message: title,
            view,
        }
    }

    fn set_selected(&mut self, selected: bool) {
        let text_alpha: CGFloat = if selected { 0.95 } else { 0.4 };
        let background_alpha: CGFloat = if selected {
            MENU_BACKGROUND_ALPHA
        } else {
            MENU_INACTIVE_ALPHA
        };

        let foreground = gui::colours::white_with_alpha(1. as CGFloat, text_alpha);
        let background = gui::colours::white_with_alpha(0. as CGFloat, background_alpha);

        unsafe {
            let _: () = msg_send![self.view, setTitleColor: foreground forState: 0 as NSUInteger];
            let _: () = msg_send![self.view, setBackgroundColor: background];
        }
    }
}

// bug: The pause-reset system sometimes allows the game to unpause within the pause menu (unless that's a game bug).
fn set_game_timer_paused(want_pause: bool) {
    // Previously, opening and closing the CLEO menu inside the pause menu would unpause the game, because we use
    //  the same mechanism as the pause menu for pausing the game (so when CLEO unpaused the game, it undid the pause
    //  menu's pausing). To stop this being possible, we use PAUSED_ALREADY to say whether the game was paused when
    //  CLEO first tried to pause it (on opening the menu). Then, when CLEO tries to unpause the game, it only happens
    //  if the game was not paused already. This means that users can't unpause the game when the game itself wants to
    #[cfg(target_pointer_width = "64")]
    {
        static mut PAUSED_ALREADY: bool = false;

        let var_ptr = crate::hook::slide::<*mut bool>(0x1007d3b34);

        if want_pause {
            let currently_paused: bool = crate::hook::deref_global(0x1007d3b34);

            if currently_paused {
                unsafe {
                    PAUSED_ALREADY = true;
                }

                return;
            }

            unsafe {
                PAUSED_ALREADY = false;
                *var_ptr = true;
            }
        } else {
            // We want to unpause.

            if unsafe { PAUSED_ALREADY } {
                // Don't do anything, because the game was paused when we found it.
                log::info!("Game was paused when found, so not unpausing.");
                return;
            }

            // The game wasn't paused, so we can unpause it.
            unsafe {
                *var_ptr = false;
            }
        }
    }

    #[cfg(target_pointer_width = "32")]
    {
        let _ = want_pause;
    }
}

// hack: Using names for TabState structures will not work if the tab changes its contents.
// todo: Remember the selected tab.
static mut TAB_STATES: Lazy<HashMap<MessageKey, TabState>> = Lazy::new(HashMap::new);

struct Menu {
    tabs: Vec<Tab>,
    tab_buttons: Vec<TabButton>,
    close_button: *mut Object,
    blur_view: *mut Object,
}

/// Creates a new UIBlurEffect object with the given UIBlurEffectStyle value. The values are 0 for
/// extra light, 1 for light, 2 for dark, 3 for extra dark, 4 for regular and 5 for prominent.
fn create_blur_effect(number: u32) -> *mut Object {
    let uiblureffect = class!(UIBlurEffect);
    unsafe { msg_send![uiblureffect, effectWithStyle: number] }
}

/// Creates a new visual effect view with a blur. See `create_blur_effect` for details on
/// `blur_mode`.
fn create_blur_view(frame: CGRect, blur_mode: u32) -> *mut Object {
    let effect = create_blur_effect(blur_mode);

    let view = unsafe {
        let view: *mut Object = msg_send![class!(UIVisualEffectView), alloc];
        let view: *mut Object = msg_send![view, initWithEffect: effect];
        let _: () = msg_send![view, setFrame: frame];
        let _: () = msg_send![view, setAutoresizingMask: 18 as NSUInteger];

        view
    };

    view
}

/// Returns the view that the CLEO menu should be added to.
/// Attaching to `rootViewController.view` ensures UIKit's landscape rotation
/// transform is properly inherited on iOS (especially 32-bit iOS 7-10 where `UIWindow`
/// remains in portrait coordinate space).
unsafe fn get_menu_container() -> *mut Object {
    let application: *mut Object = msg_send![class!(UIApplication), sharedApplication];
    if application.is_null() {
        return std::ptr::null_mut();
    }

    let mut window: *mut Object = msg_send![application, keyWindow];
    if window.is_null() {
        let windows: *mut Object = msg_send![application, windows];
        if !windows.is_null() {
            let count: NSUInteger = msg_send![windows, count];
            if count > 0 {
                window = msg_send![windows, objectAtIndex: 0 as NSUInteger];
            }
        }
    }

    if window.is_null() {
        return std::ptr::null_mut();
    }

    let mut vc: *mut Object = msg_send![window, rootViewController];
    while !vc.is_null() {
        let presented: *mut Object = msg_send![vc, presentedViewController];
        if presented.is_null() {
            break;
        }
        vc = presented;
    }

    if !vc.is_null() {
        let view: *mut Object = msg_send![vc, view];
        if !view.is_null() {
            return view;
        }
    }

    window
}

impl Menu {
    fn new(tab_data: Vec<TabData>) -> Menu {
        let language = super::language::current();

        #[cfg(target_pointer_width = "64")]
        let (screen_w, screen_h) = unsafe {
            let container = get_menu_container();
            if !container.is_null() {
                let raw_bounds: CGRect = msg_send![container, bounds];
                let w = raw_bounds.size.width.max(raw_bounds.size.height);
                let h = raw_bounds.size.width.min(raw_bounds.size.height);
                (w, h)
            } else {
                let screen: *mut Object = msg_send![class!(UIScreen), mainScreen];
                let raw_bounds: CGRect = msg_send![screen, bounds];
                let w = raw_bounds.size.width.max(raw_bounds.size.height);
                let h = raw_bounds.size.width.min(raw_bounds.size.height);
                (w, h)
            }
        };

        #[cfg(target_pointer_width = "32")]
        let (screen_w, screen_h) = (568.0 as CGFloat, 320.0 as CGFloat);

        let frame = CGRect::new(0.0 as CGFloat, 0.0 as CGFloat, screen_w, screen_h);

        log::info!("Creating CLEO menu with frame: {:.1}x{:.1}", screen_w, screen_h);

        let tab_btn_width = frame.size.width / (tab_data.len() as CGFloat);

        let tab_buttons: Vec<_> = tab_data
            .iter()
            .enumerate()
            .map(|(index, data)| TabButton::new(data.name.clone(), index, tab_btn_width))
            .collect();

        let tab_frame = CGRect::new(
            0.,
            TAB_BUTTON_HEIGHT,
            frame.size.width,
            frame.size.height - (TAB_BUTTON_HEIGHT + CLOSE_BUTTON_HEIGHT),
        );

        // Move all the tab data into Tab structures.
        let tabs = tab_data.into_iter().enumerate().map(|(tab_index, data)| {
            let state = unsafe {
                // We remove the existing state from the map if it exists, because we need to move it into
                //  the Tab we're about to create. It will be returned to the map when the menu is closed.
                TAB_STATES.remove(&data.name.key()).unwrap_or(TabState {
                    selected: false,
                    scroll_y: 0.,
                })
            };

            let tab = Tab::new(data, tab_frame, state);

            for (row_index, row) in tab.rows.iter().enumerate() {
                add_button_handler(row.button, ButtonTag::new_row(tab_index, row_index));
            }

            tab
        });

        // We collect here instead of chaining so that the formatting above is nicer.
        let tabs: Vec<_> = tabs.collect();

        let close_message = MessageKey::MenuClose.to_message();

        let close_button: *mut Object = unsafe {
            let btn: *mut Object = msg_send![class!(UIButton), alloc];

            let btn_frame = CGRect::new(
                0.,
                frame.size.height - CLOSE_BUTTON_HEIGHT,
                frame.size.width,
                CLOSE_BUTTON_HEIGHT,
            );

            let btn: *mut Object = msg_send![btn, initWithFrame: btn_frame];

            let close_string = ns_string(close_message.translate());

            let _: () = msg_send![btn, setTitle: close_string forState: 0 as NSUInteger];
            let _: () = msg_send![
                btn,
                setBackgroundColor: gui::colours::get(gui::colours::RED, 0.35 as CGFloat)
            ];

            let label: *mut Object = msg_send![btn, titleLabel];

            let font = language.font_set().title_font.uifont(CLOSE_BTN_FONT_SIZE);
            let _: () = msg_send![label, setFont: font];

            add_button_handler(btn, ButtonTag::new_close());

            btn
        };

        // Create the blur view to hold the menu components. The blur is important because it
        // allows the menu to be translucent without ruining the readability of the text.
        let blur_view = create_blur_view(frame, 3);

        // Add everything to the blur view.
        unsafe {
            let content_view: *mut Object = msg_send![blur_view, contentView];

            for tab_button in &tab_buttons {
                let _: () = msg_send![content_view, addSubview: tab_button.view];
            }

            for tab in &tabs {
                let _: () = msg_send![content_view, addSubview: tab.scroll_view];

                if let Some(label) = tab.warning_label {
                    let _: () = msg_send![content_view, addSubview: label];
                }
            }

            let _: () = msg_send![content_view, addSubview: close_button];
        }

        Menu {
            tabs,
            tab_buttons,
            close_button,
            blur_view,
        }
    }

    fn add_to_window(&mut self) {
        set_game_timer_paused(true);

        unsafe {
            let container = get_menu_container();
            if !container.is_null() {
                #[cfg(target_pointer_width = "64")]
                let (screen_w, screen_h) = {
                    let bounds: CGRect = msg_send![container, bounds];
                    (bounds.size.width.max(bounds.size.height), bounds.size.width.min(bounds.size.height))
                };
                #[cfg(target_pointer_width = "32")]
                let (screen_w, screen_h) = (568.0 as CGFloat, 320.0 as CGFloat);

                let frame = CGRect::new(0.0 as CGFloat, 0.0 as CGFloat, screen_w, screen_h);
                let _: () = msg_send![self.blur_view, setFrame: frame];

                let _: () = msg_send![container, addSubview: self.blur_view];
                let _: () = msg_send![container, bringSubviewToFront: self.blur_view];
                log::info!(
                    "CLEO menu added to container view: {:?} with frame {:.1}x{:.1}",
                    container,
                    screen_w,
                    screen_h
                );
            } else {
                log::error!("Failed to get container for CLEO menu!");
            }
        }

        for i in 0..self.tabs.len() {
            self.tabs[i].set_selected(i == 0);
            self.tab_buttons[i].set_selected(i == 0);
        }

        log::info!("CLEO menu displayed.");
    }

    fn remove(self) {
        unsafe {
            for tab_button in &self.tab_buttons {
                let _: () = msg_send![tab_button.view, removeFromSuperview];
            }

            for tab in &self.tabs {
                let content_offset: CGPoint = msg_send![tab.scroll_view, contentOffset];

                // todo: Add tab selection state to TAB_STATES when menu is removed.
                TAB_STATES.insert(
                    tab.name.key(),
                    TabState {
                        selected: false,
                        scroll_y: content_offset.y,
                    },
                );

                let _: () = msg_send![tab.scroll_view, removeFromSuperview];

                if let Some(label) = tab.warning_label {
                    let _: () = msg_send![label, removeFromSuperview];
                }
            }

            let _: () = msg_send![self.close_button, removeFromSuperview];
            let _: () = msg_send![self.blur_view, removeFromSuperview];
        }

        set_game_timer_paused(false);
    }

    fn get_module_tab_data() -> Vec<TabData> {
        #[cfg(target_pointer_width = "64")]
        let game_state = unsafe { *crate::hook::slide::<*const u32>(0x1006806d0) };

        #[cfg(target_pointer_width = "32")]
        let game_state = 9u32; // In game, allow all tabs on iPhone 5

        // The menu will be automatically created from any TabData structures in the
        //  vector that we return, so adding another tab simply requires adding other stuff here.
        if game_state == 9 {
            // In a game, so allow access to all the tabs.
            vec![
                crate::game::scripts::runtime::tab_data_csi(),
                crate::game::scripts::runtime::tab_data_csa(),
                crate::game::cheats::tab_data(),
                crate::meta::settings::tab_data(),
            ]
        } else {
            // Not in a game, so only allow access to the settings tab.
            vec![crate::meta::settings::tab_data()]
        }
    }

    fn reload_rows(&mut self) {
        for tab in &mut self.tabs {
            tab.reload_warning();
            tab.reload_fonts();

            for row in &mut tab.rows {
                row.load();
            }
        }

        for tab_button in &mut self.tab_buttons {
            tab_button.reload_title();
        }

        let close_title_objc = ns_string(MessageKey::MenuClose.to_message().translate());
        let close_font = super::language::current()
            .font_set()
            .title_font
            .uifont(CLOSE_BTN_FONT_SIZE);

        unsafe {
            let _: () = msg_send![self.close_button, setTitle: close_title_objc forState: 0 as NSUInteger];

            let label: *mut Object = msg_send![self.close_button, titleLabel];
            let _: () = msg_send![label, setFont: close_font];
        }
    }

    fn rebuild_tab(&mut self, tab_index: usize) {
        if tab_index >= self.tabs.len() {
            return;
        }

        let mut module_tabs = Self::get_module_tab_data();
        if tab_index >= module_tabs.len() {
            return;
        }

        let new_tab_data = module_tabs.remove(tab_index);
        self.tabs[tab_index].rebuild_rows(new_tab_data, tab_index);
    }

    fn start_channel_polling() -> Sender<MenuMessage> {
        let (sender, receiver) = mpsc::channel();

        // fixme: start_channel_polling contains lots of prototyping code that needs to be updated.
        std::thread::spawn(move || {
            type MenuMutex = Mutex<Option<Menu>>;
            let menu: Arc<MenuMutex> = Arc::new(Mutex::new(None));
            unsafe impl Send for Menu {}

            fn do_on_ui_thread<F, T>(f: F) -> T
            where
                F: FnOnce() -> T,
                F: Send + 'static,
                T: Send + 'static,
            {
                dispatch::Queue::main().exec_sync(f)
            }

            loop {
                match receiver.recv().expect("recv() for menu channel failed") {
                    MenuMessage::Show => {
                        log::info!("Showing CLEO menu on UI thread...");

                        let menu = Arc::clone(&menu);

                        do_on_ui_thread(move || {
                            let mut menu = menu.lock().unwrap();

                            if menu.is_none() {
                                *menu = Some(Menu::new(Self::get_module_tab_data()));
                                menu.as_mut().unwrap().add_to_window();
                                log::info!("CLEO menu successfully presented.");
                            } else {
                                log::warn!("Menu already exists, but was activated again (which should be impossible)");
                            }
                        });
                    }

                    MenuMessage::Hide => {
                        if menu.lock().unwrap().is_some() {
                            let menu = Arc::clone(&menu);

                            do_on_ui_thread(move || {
                                menu.lock().unwrap().take().unwrap().remove();
                            });
                        }
                    }

                    MenuMessage::ReloadRows => {
                        if menu.lock().unwrap().is_some() {
                            let menu = Arc::clone(&menu);

                            do_on_ui_thread(move || {
                                let mut menu = menu.lock().unwrap();
                                let menu = menu.as_mut().unwrap();

                                menu.reload_rows();
                            });
                        }
                    }

                    MenuMessage::RebuildTab(tab_index) => {
                        if menu.lock().unwrap().is_some() {
                            let menu = Arc::clone(&menu);

                            do_on_ui_thread(move || {
                                let mut menu = menu.lock().unwrap();
                                let menu = menu.as_mut().unwrap();

                                menu.rebuild_tab(tab_index);
                            });
                        }
                    }

                    MenuMessage::SelectTab(tab_index) => {
                        if menu.lock().unwrap().is_some() {
                            let menu = Arc::clone(&menu);

                            do_on_ui_thread(move || {
                                let mut menu = menu.lock().unwrap();
                                let menu = menu.as_mut().unwrap();

                                for (index, tab) in menu.tabs.iter_mut().enumerate() {
                                    tab.set_selected(index == tab_index);
                                }

                                for (index, tab_button) in menu.tab_buttons.iter_mut().enumerate() {
                                    tab_button.set_selected(index == tab_index);
                                }
                            });
                        } else {
                            log::warn!("Tab select message delivered, but menu does not exist!");
                        }
                    }

                    MenuMessage::HitRow(tab_index, row_index) => {
                        if menu.lock().unwrap().is_some() {
                            let menu = Arc::clone(&menu);

                            do_on_ui_thread(move || {
                                let mut menu = menu.lock().unwrap();
                                let menu = menu.as_mut().unwrap();
                                let row = &mut menu.tabs[tab_index].rows[row_index];
                                row.hit();
                            });
                        } else {
                            log::warn!("Tab select message delivered, but menu does not exist!");
                        }
                    }
                }
            }
        });

        sender
    }
}

#[cfg(target_pointer_width = "64")]
fn get_reachability_class() -> *const Object {
    unsafe {
        let cls = objc::runtime::objc_getClass(b"Reachability\0".as_ptr() as *const i8);
        if !cls.is_null() {
            cls as *const Object
        } else {
            let cls = objc::runtime::objc_getClass(b"IOSReachability\0".as_ptr() as *const i8);
            if !cls.is_null() {
                cls as *const Object
            } else {
                panic!("Neither Reachability nor IOSReachability class found!");
            }
        }
    }
}

#[cfg(target_pointer_width = "64")]
fn reachability_with_hostname(
    this_class: *const Object,
    sel: objc::runtime::Sel,
    hostname: *mut Object,
) -> *mut Object {
    unsafe {
        let is_button: bool = msg_send![hostname, isKindOfClass: class!(UIButton)];

        if is_button {
            let raw_tag: NSInteger = msg_send![hostname, tag];
            let tag = ButtonTag::from_ns_integer(raw_tag);

            if tag.is_close {
                log::trace!("Close button pressed");
                MenuMessage::Hide.send();
                return std::ptr::null_mut();
            }

            if tag.tab == -1 {
                log::error!("tag.tab cannot be -1 when tag.is_close is false");
            } else if tag.row == -1 {
                MenuMessage::SelectTab(tag.tab as usize).send();
            } else {
                MenuMessage::HitRow(tag.tab as usize, tag.row as usize).send();
            }

            std::ptr::null_mut()
        } else {
            crate::call_original!(crate::targets::button_hack, this_class, sel, hostname)
        }
    }
}

#[cfg(target_pointer_width = "64")]
fn add_button_handler(button: *mut Object, tag: ButtonTag) {
    let reachability = get_reachability_class();
    let selector = sel!(reachabilityWithHostName:);
    let touch_up_inside = (1 << 6) as NSUInteger;

    unsafe {
        let _: () = msg_send![button, setTag: tag.to_ns_integer()];
        let _: () = msg_send![button, addTarget: reachability action: selector forControlEvents: touch_up_inside];
    }
}

#[cfg(target_pointer_width = "32")]
mod button_target_32 {
    use super::*;
    use objc::runtime::{Class, Object, Sel, BOOL};

    extern "C" {
        fn objc_allocateClassPair(superclass: *const Class, name: *const libc::c_char, extraBytes: usize) -> *mut Class;
        fn objc_registerClassPair(cls: *mut Class);
        fn class_addMethod(cls: *mut Class, name: Sel, imp: extern "C" fn(*mut Object, Sel, *mut Object), types: *const libc::c_char) -> BOOL;
    }

    extern "C" fn button_clicked(_this: *mut Object, _cmd: Sel, sender: *mut Object) {
        unsafe {
            let is_button: bool = msg_send![sender, isKindOfClass: class!(UIButton)];
            if is_button {
                let raw_tag: NSInteger = msg_send![sender, tag];
                let tag = ButtonTag::from_ns_integer(raw_tag);

                if tag.is_close {
                    log::trace!("Close button pressed");
                    MenuMessage::Hide.send();
                    return;
                }

                if tag.tab == -1 {
                    log::error!("tag.tab cannot be -1 when tag.is_close is false");
                } else if tag.row == -1 {
                    MenuMessage::SelectTab(tag.tab as usize).send();
                } else {
                    MenuMessage::HitRow(tag.tab as usize, tag.row as usize).send();
                }
            }
        }
    }

    static BUTTON_TARGET: OnceCell<usize> = OnceCell::new();

    pub fn get_target() -> *mut Object {
        *BUTTON_TARGET.get_or_init(|| {
            unsafe {
                let class_name = b"CleoButtonTarget\0".as_ptr() as *const libc::c_char;
                let mut cls = objc::runtime::objc_getClass(class_name) as *mut Class;
                if cls.is_null() {
                    let superclass = class!(NSObject);
                    cls = objc_allocateClassPair(superclass, class_name, 0);
                    let types = b"v@:@\0".as_ptr() as *const libc::c_char;
                    class_addMethod(cls, sel!(handleButton:), button_clicked, types);
                    objc_registerClassPair(cls);
                }
                let target: *mut Object = msg_send![cls, new];
                target as usize
            }
        }) as *mut Object
    }
}

#[cfg(target_pointer_width = "32")]
fn add_button_handler(button: *mut Object, tag: ButtonTag) {
    let target = button_target_32::get_target();
    let selector = sel!(handleButton:);
    let touch_up_inside = (1 << 6) as NSUInteger;

    unsafe {
        let _: () = msg_send![button, setTag: tag.to_ns_integer()];
        let _: () = msg_send![button, addTarget: target action: selector forControlEvents: touch_up_inside];
    }
}

pub fn init() {
    log::info!("installing menu hook...");

    #[cfg(target_pointer_width = "64")]
    crate::targets::button_hack::install(reachability_with_hostname);

    #[cfg(target_pointer_width = "32")]
    let _ = button_target_32::get_target();

    log::info!("starting menu poll...");
    MESSAGE_SENDER
        .set(Mutex::new(Menu::start_channel_polling()))
        .unwrap();
}

impl Drop for Row {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.detail_label, release];
            let _: () = msg_send![self.value_label, release];
            let _: () = msg_send![self.button, release];
        }
    }
}

impl Drop for Tab {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.scroll_view, release];

            if let Some(label) = self.warning_label {
                let _: () = msg_send![label, release];
            }
        }
    }
}

impl Drop for TabButton {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.view, release];
        }
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.close_button, release];
            let _: () = msg_send![self.blur_view, release];
        }
    }
}
