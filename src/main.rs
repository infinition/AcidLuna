#![windows_subsystem = "windows"]
use std::{thread, time::Duration};
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM, HINSTANCE};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, mouse_event, 
    VK_LCONTROL, VK_CAPITAL, 
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, SetWindowsHookExA, GetMessageA, DispatchMessageA,
    WH_KEYBOARD_LL, HC_ACTION, WM_KEYDOWN, WM_SYSKEYDOWN, KBDLLHOOKSTRUCT, HHOOK
};
use tray_icon::{TrayIconBuilder, TrayIconEvent};
use tray_icon::menu::{Menu, MenuItem, MenuEvent};

// Variable globale pour stocker le Hook (nécessaire pour la callback)
static mut H_HOOK: HHOOK = HHOOK(0);

// Cette fonction intercepte toutes les touches AVANT Windows
unsafe extern "system" fn keyboard_hook_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        let kbd_struct = unsafe { *(l_param.0 as *const KBDLLHOOKSTRUCT) };
        
        // Si la touche interceptée est VERR MAJ (VK_CAPITAL)
        if kbd_struct.vkCode == VK_CAPITAL.0 as u32 {
            
            // Si c'est un appui sur la touche (KEYDOWN)
            if w_param.0 as u32 == WM_KEYDOWN || w_param.0 as u32 == WM_SYSKEYDOWN {
                // On déclenche le Clic Droit
                unsafe {
                    mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
                    mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
                }
            }

            // IMPORTANT : On retourne 1 pour dire à Windows "J'ai traité cette touche, oublie-la".
            // Cela empêche l'activation du mode MAJUSCULE.
            return LRESULT(1);
        }
    }
    
    // Pour toutes les autres touches, on laisse passer
    unsafe { CallNextHookEx(H_HOOK, code, w_param, l_param) }
}

fn create_generic_icon() -> tray_icon::Icon {
    let width = 32;
    let height = 32;
    let mut rgba = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            // Créer un simple cercle blanc pour simuler une lune
            let dx = x as i32 - 16;
            let dy = y as i32 - 16;
            let distance_sq = dx * dx + dy * dy;
            if distance_sq < 14 * 14 {
                rgba.extend_from_slice(&[255, 255, 255, 255]);
            } else {
                rgba.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    tray_icon::Icon::from_rgba(rgba, width, height).expect("Failed to create generic icon")
}

fn main() {
    // --- CRÉATION DE L'ICÔNE GÉNÉRIQUE ---
    let icon = create_generic_icon();

    // --- CRÉATION DU MENU ---
    let tray_menu = Menu::new();
    let quit_i = MenuItem::new("Fermer", true, None);
    tray_menu.append(&quit_i).unwrap();

    // --- CRÉATION DE LA TRAY ICON ---
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("AcidLuna - iPad Support")
        .with_icon(icon)
        .build()
        .unwrap();

    // --- THREAD 1 : GESTION DU DRAG (CTRL) ---
    thread::spawn(|| {
        let mut is_dragging = false;
        loop {
            unsafe {
                // On lit l'état de CTRL
                let ctrl_state = GetAsyncKeyState(VK_LCONTROL.0 as i32) as i16;
                let is_ctrl_down = ctrl_state < 0; // Bit de poids fort indique l'appui

                if is_ctrl_down && !is_dragging {
                    mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                    is_dragging = true;
                } 
                else if !is_ctrl_down && is_dragging {
                    mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    is_dragging = false;
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    // --- THREAD PRINCIPAL : GESTION DU HOOK ET DES ÉVÉNEMENTS TRAY ---
    unsafe {
        // Installation du hook clavier bas niveau
        H_HOOK = SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(keyboard_hook_proc),
            HINSTANCE(0),
            0
        ).expect("Impossible d'installer le Hook clavier");

        // Boucle de messages Windows
        let mut msg = std::mem::zeroed();
        while GetMessageA(&mut msg, None, 0, 0).into() {
            // Gestion des événements de la tray icon
            if let Ok(_event) = TrayIconEvent::receiver().try_recv() {
                // Événements tray ignorés pour l'instant
            }
            
            // Gestion des événements du menu
            if let Ok(event) = MenuEvent::receiver().try_recv() {
                if event.id == quit_i.id() {
                    std::process::exit(0);
                }
            }

            DispatchMessageA(&msg);
        }
    }
}



