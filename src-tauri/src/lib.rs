use tauri::{
    webview::{PageLoadEvent, PermissionResponse, WebviewWindowBuilder},
    Manager, Theme, Url, WebviewUrl,
};

const INJECTION_SCRIPT: &str = r#"
(function() {
  if (window.location.hostname === 'myaccount.google.com' || window.location.href.indexOf('myaccount.google.com') !== -1) {
    window.location.replace('https://mypixelbuds.google.com');
    return;
  }

  const css = `
    header#gb, .gb_Ta, .gb_Vd {
      padding-right: 140px !important;
      -webkit-app-region: drag !important;
    }

    #pixelbuds-window-controls,
    #pixelbuds-window-controls *,
    header#gb button,
    header#gb a,
    header#gb input,
    header#gb [role="button"] {
      -webkit-app-region: no-drag !important;
    }

    #pixelbuds-window-controls {
      position: fixed !important;
      top: 10px !important;
      right: 10px !important;
      height: 40px !important;
      z-index: 2147483647 !important;
      display: flex !important;
      align-items: center !important;
      gap: 2px !important;
      background: transparent !important;
      border: none !important;
      box-shadow: none !important;
      backdrop-filter: none !important;
      -webkit-backdrop-filter: none !important;
      padding: 0 !important;
      margin: 0 !important;
      pointer-events: auto !important;
      opacity: 1 !important;
      visibility: visible !important;
      user-select: none !important;
      -webkit-user-select: none !important;
    }

    #pixelbuds-window-controls button {
      width: 40px !important;
      height: 40px !important;
      border-radius: 50% !important;
      border: none !important;
      background: transparent !important;
      color: #ffffff !important;
      display: inline-flex !important;
      align-items: center !important;
      justify-content: center !important;
      cursor: pointer !important;
      padding: 0 !important;
      margin: 0 !important;
      outline: none !important;
      box-shadow: none !important;
      transition: none !important;
      opacity: 1 !important;
    }

    #pixelbuds-window-controls button:hover,
    #pixelbuds-window-controls button:active,
    #pixelbuds-window-controls button:focus,
    #pixelbuds-window-controls button#pb-btn-close:hover {
      background: transparent !important;
      color: #ffffff !important;
      opacity: 1 !important;
      outline: none !important;
      box-shadow: none !important;
    }

    #pixelbuds-window-controls button svg {
      display: block !important;
      width: 24px !important;
      height: 24px !important;
      pointer-events: none !important;
    }

    #pixelbuds-window-controls button svg * {
      stroke: #ffffff !important;
      fill: none !important;
      pointer-events: none !important;
    }

    #gbwa,
    .gb_L.gb_Hd.gb_aa {
      display: none !important;
      visibility: hidden !important;
      pointer-events: none !important;
    }

    #ogb_feedback_control {
      display: none !important;
      visibility: hidden !important;
      pointer-events: none !important;
    }

    a[href*="support.google.com"],
    a[href*="p=web_app"] {
      display: none !important;
      visibility: hidden !important;
      pointer-events: none !important;
    }

    a[href*="myaccount.google.com"] {
      display: none !important;
      visibility: hidden !important;
      pointer-events: none !important;
    }

    .mat-mdc-button-persistent-ripple,
    .mdc-button__ripple {
      display: none !important;
      visibility: hidden !important;
      opacity: 0 !important;
      pointer-events: none !important;
    }
  `;

  function setupHeaderDrag() {
    const header = document.querySelector('header#gb') || document.querySelector('.gb_Ta');
    if (header && !header.dataset.dragInitialized) {
      header.dataset.dragInitialized = 'true';
      header.setAttribute('data-tauri-drag-region', '');
      header.addEventListener('mousedown', function(e) {
        if (e.target.closest('#pixelbuds-window-controls, button, a, input, [role="button"]')) {
          return;
        }
        if (e.button === 0) {
          if (e.detail === 2) {
            window.location.href = 'https://pixelbuds-action.local/maximize?t=' + Date.now();
          } else {
            window.location.href = 'https://pixelbuds-action.local/drag?t=' + Date.now();
          }
        }
      });
    }
  }

  function injectWindowControls() {
    const body = document.body || document.documentElement;
    if (!body) return;

    let controls = document.getElementById('pixelbuds-window-controls');
    if (controls) {
      if (controls.dataset.version !== '5') {
        controls.remove();
        controls = null;
      } else {
        if (document.body && controls.parentElement !== document.body) {
          document.body.appendChild(controls);
        }
        return;
      }
    }

    controls = document.createElement('div');
    controls.id = 'pixelbuds-window-controls';
    controls.dataset.version = '5';

    const btnMin = document.createElement('button');
    btnMin.id = 'pb-btn-min';
    btnMin.type = 'button';
    const svgMin = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svgMin.setAttribute('width', '24');
    svgMin.setAttribute('height', '24');
    svgMin.setAttribute('viewBox', '0 0 24 24');
    svgMin.setAttribute('fill', 'none');
    svgMin.setAttribute('stroke', '#ffffff');
    svgMin.setAttribute('stroke-width', '2');
    svgMin.setAttribute('stroke-linecap', 'round');
    const lineMin = document.createElementNS('http://www.w3.org/2000/svg', 'line');
    lineMin.setAttribute('x1', '6');
    lineMin.setAttribute('y1', '12');
    lineMin.setAttribute('x2', '18');
    lineMin.setAttribute('y2', '12');
    svgMin.appendChild(lineMin);
    btnMin.appendChild(svgMin);

    const btnMax = document.createElement('button');
    btnMax.id = 'pb-btn-max';
    btnMax.type = 'button';
    const svgMax = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svgMax.setAttribute('width', '24');
    svgMax.setAttribute('height', '24');
    svgMax.setAttribute('viewBox', '0 0 24 24');
    svgMax.setAttribute('fill', 'none');
    svgMax.setAttribute('stroke', '#ffffff');
    svgMax.setAttribute('stroke-width', '2');
    svgMax.setAttribute('stroke-linecap', 'round');
    svgMax.setAttribute('stroke-linejoin', 'round');
    const pathMax = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    pathMax.setAttribute('d', 'M6 6h12v12H6z');
    pathMax.setAttribute('fill', 'none');
    pathMax.setAttribute('stroke', '#ffffff');
    pathMax.setAttribute('stroke-width', '2');
    svgMax.appendChild(pathMax);
    btnMax.appendChild(svgMax);

    const btnClose = document.createElement('button');
    btnClose.id = 'pb-btn-close';
    btnClose.type = 'button';
    const svgClose = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svgClose.setAttribute('width', '24');
    svgClose.setAttribute('height', '24');
    svgClose.setAttribute('viewBox', '0 0 24 24');
    svgClose.setAttribute('fill', 'none');
    svgClose.setAttribute('stroke', '#ffffff');
    svgClose.setAttribute('stroke-width', '2');
    svgClose.setAttribute('stroke-linecap', 'round');
    const lClose1 = document.createElementNS('http://www.w3.org/2000/svg', 'line');
    lClose1.setAttribute('x1', '6');
    lClose1.setAttribute('y1', '6');
    lClose1.setAttribute('x2', '18');
    lClose1.setAttribute('y2', '18');
    const lClose2 = document.createElementNS('http://www.w3.org/2000/svg', 'line');
    lClose2.setAttribute('x1', '18');
    lClose2.setAttribute('y1', '6');
    lClose2.setAttribute('x2', '6');
    lClose2.setAttribute('y2', '18');
    svgClose.appendChild(lClose1);
    svgClose.appendChild(lClose2);
    btnClose.appendChild(svgClose);

    controls.appendChild(btnMin);
    controls.appendChild(btnMax);
    controls.appendChild(btnClose);

    body.appendChild(controls);

    function sendAction(action) {
      window.location.href = 'https://pixelbuds-action.local/' + action + '?t=' + Date.now();
    }

    btnMin.addEventListener('click', function(e) {
      e.preventDefault();
      e.stopPropagation();
      e.stopImmediatePropagation();
      sendAction('minimize');
    }, true);

    btnMax.addEventListener('click', function(e) {
      e.preventDefault();
      e.stopPropagation();
      e.stopImmediatePropagation();
      sendAction('maximize');
    }, true);

    btnClose.addEventListener('click', function(e) {
      e.preventDefault();
      e.stopPropagation();
      e.stopImmediatePropagation();
      sendAction('close');
    }, true);
  }

  function applyCustomStyles() {
    if (window.location.hostname === 'myaccount.google.com' || window.location.href.indexOf('myaccount.google.com') !== -1) {
      window.location.replace('https://mypixelbuds.google.com');
      return;
    }

    let style = document.getElementById('pixelbuds-custom-injections');
    if (!style) {
      style = document.createElement('style');
      style.id = 'pixelbuds-custom-injections';
      style.textContent = css;
      (document.head || document.documentElement).appendChild(style);
    }

    const bwContainers = document.querySelectorAll('.bwIQH');
    bwContainers.forEach(function(c) {
      if (c.style.display === 'none') {
        c.style.removeProperty('display');
      }
    });

    const gbwa = document.getElementById('gbwa');
    if (gbwa) {
      gbwa.style.setProperty('display', 'none', 'important');
    }

    const feedback = document.querySelectorAll('#ogb_feedback_control');
    feedback.forEach(function(el) {
      el.style.setProperty('display', 'none', 'important');
    });

    const supportButtons = document.querySelectorAll('a[href*="support.google.com"], a[href*="p=web_app"]');
    supportButtons.forEach(function(el) {
      el.style.setProperty('display', 'none', 'important');
    });

    const myAccountButtons = document.querySelectorAll('a[href*="myaccount.google.com"]');
    myAccountButtons.forEach(function(el) {
      el.style.setProperty('display', 'none', 'important');
      if (el.parentElement && el.parentElement.tagName === 'SPAN' && el.parentElement.classList.contains('iz5qmd')) {
        el.parentElement.style.setProperty('display', 'none', 'important');
      }
    });

    const keepVisible = document.querySelectorAll('a[href*="Logout"], a[href*="AddSession"], a[href*="SignOutOptions"]');
    keepVisible.forEach(function(el) {
      el.style.removeProperty('display');
      if (el.parentElement) {
        el.parentElement.style.removeProperty('display');
      }
    });

    const ripples = document.querySelectorAll('.mat-mdc-button-persistent-ripple, .mdc-button__ripple');
    ripples.forEach(function(el) {
      el.style.setProperty('display', 'none', 'important');
    });

    injectWindowControls();
    setupHeaderDrag();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyCustomStyles);
  } else {
    applyCustomStyles();
  }

  const observer = new MutationObserver(function() {
    applyCustomStyles();
  });

  const target = document.body || document.documentElement;
  if (target) {
    observer.observe(target, { childList: true, subtree: true });
  } else {
    window.addEventListener('DOMContentLoaded', function() {
      observer.observe(document.body || document.documentElement, { childList: true, subtree: true });
    });
  }
})();
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .on_permission_request(|_webview, _kind| {
            PermissionResponse::Allow
        })
        .setup(|app| {
            let app_handle = app.handle().clone();
            let external_url = Url::parse("https://mypixelbuds.google.com").unwrap();
            let _window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External(external_url),
            )
            .title("Pixel Buds")
            .decorations(false)
            .theme(Some(Theme::Dark))
            .inner_size(1024.0, 768.0)
            .min_inner_size(640.0, 500.0)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36")
            .additional_browser_args("--enable-features=WebBluetooth,WebBluetoothNewPermissionsBackend --enable-web-bluetooth")
            .initialization_script(INJECTION_SCRIPT)
            .on_navigation(move |url| {
                let url_str = url.as_str();
                if url.host_str() == Some("myaccount.google.com") {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.navigate(Url::parse("https://mypixelbuds.google.com").unwrap());
                    }
                    return false;
                }
                if url.scheme() == "pixelbuds-action" || url.host_str() == Some("pixelbuds-action.local") {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        if url_str.contains("minimize") {
                            let _ = window.minimize();
                        } else if url_str.contains("maximize") {
                            if window.is_maximized().unwrap_or(false) {
                                let _ = window.unmaximize();
                            } else {
                                let _ = window.maximize();
                            }
                        } else if url_str.contains("close") {
                            let _ = window.close();
                        } else if url_str.contains("drag") {
                            let _ = window.start_dragging();
                        }
                    }
                    return false;
                }
                true
            })
            .on_page_load(|window, payload| {
                if payload.event() == PageLoadEvent::Finished {
                    if let Ok(current_url) = window.url() {
                        if current_url.host_str() == Some("myaccount.google.com") {
                            let _ = window.navigate(Url::parse("https://mypixelbuds.google.com").unwrap());
                            return;
                        }
                    }
                    let _ = window.eval(INJECTION_SCRIPT);
                }
            })
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
