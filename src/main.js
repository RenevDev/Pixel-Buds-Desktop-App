function log(message, type = 'info') {
  const logContent = document.getElementById('log-content');
  if (!logContent) return;
  const time = new Date().toLocaleTimeString();
  const color = type === 'error' ? '#f28b82' : type === 'success' ? '#81c995' : '#c4d8fd';
  logContent.innerHTML += `<div style="color: ${color}">[${time}] ${message}</div>`;
  logContent.scrollTop = logContent.scrollHeight;
}

async function checkEnvironment() {
  const badgeBt = document.getElementById('badge-bluetooth');
  const descBt = document.getElementById('desc-bluetooth');
  const badgeRadio = document.getElementById('badge-radio');
  const descRadio = document.getElementById('desc-radio');
  const badgeUa = document.getElementById('badge-ua');
  const descUa = document.getElementById('desc-ua');

  // Check User-Agent
  const ua = navigator.userAgent;
  if (descUa) {
    descUa.textContent = ua;
  }
  if (ua.includes('Chrome/')) {
    badgeUa.className = 'badge badge-success';
    badgeUa.textContent = 'Chrome Mask Active';
    log('User-Agent successfully configured with Chrome identity: ' + ua, 'success');
  } else {
    badgeUa.className = 'badge badge-pending';
    badgeUa.textContent = 'Non-Chrome UA';
    log('Warning: User-Agent does not have Chrome identity: ' + ua, 'warning');
  }

  // Check Web Bluetooth API
  if ('bluetooth' in navigator) {
    badgeBt.className = 'badge badge-success';
    badgeBt.textContent = 'Available';
    descBt.textContent = 'navigator.bluetooth is present and supported.';
    log('Web Bluetooth API (navigator.bluetooth) is enabled in this WebView2 instance.', 'success');

    // Check adapter availability
    try {
      if (typeof navigator.bluetooth.getAvailability === 'function') {
        const available = await navigator.bluetooth.getAvailability();
        if (available) {
          badgeRadio.className = 'badge badge-success';
          badgeRadio.textContent = 'Ready (ON)';
          descRadio.textContent = 'Bluetooth adapter is detected and active.';
          log('Bluetooth adapter radio is ON and ready.', 'success');
        } else {
          badgeRadio.className = 'badge badge-danger';
          badgeRadio.textContent = 'Turned OFF';
          descRadio.textContent = 'Bluetooth adapter is turned off or not detected by Windows.';
          log('Bluetooth adapter appears to be disabled in Windows Settings.', 'error');
        }
      } else {
        badgeRadio.className = 'badge badge-pending';
        badgeRadio.textContent = 'Unknown';
        descRadio.textContent = 'getAvailability() not implemented, adapter check skipped.';
      }
    } catch (err) {
      badgeRadio.className = 'badge badge-danger';
      badgeRadio.textContent = 'Error';
      descRadio.textContent = 'Failed checking adapter: ' + err.message;
      log('Error querying getAvailability: ' + err.message, 'error');
    }
  } else {
    badgeBt.className = 'badge badge-danger';
    badgeBt.textContent = 'Not Found';
    descBt.textContent = 'navigator.bluetooth is undefined. Make sure browser flags are enabled.';
    badgeRadio.className = 'badge badge-danger';
    badgeRadio.textContent = 'N/A';
    descRadio.textContent = 'Cannot inspect adapter without Web Bluetooth.';
    log('navigator.bluetooth is not defined in this webview.', 'error');
  }
}

async function scanBluetooth() {
  log('Starting Bluetooth device request dialog...', 'info');
  if (!navigator.bluetooth) {
    log('Cannot scan: navigator.bluetooth is not supported.', 'error');
    alert('Web Bluetooth API is niet beschikbaar in deze browser.');
    return;
  }

  try {
    const device = await navigator.bluetooth.requestDevice({
      acceptAllDevices: true,
      optionalServices: ['battery_service', 'device_information']
    });

    log(`[Connected] Device selected: "${device.name || 'Unnamed'}" (ID: ${device.id})`, 'success');

    device.addEventListener('gattserverdisconnected', () => {
      log(`Device "${device.name || device.id}" disconnected.`, 'warning');
    });

    if (device.gatt) {
      log('Attempting GATT server connection...', 'info');
      const server = await device.gatt.connect();
      log(`GATT Server connected: ${server.connected ? 'Connected' : 'Disconnected'}`, 'success');
    }
  } catch (err) {
    if (err.name === 'NotFoundError') {
      log('Bluetooth scan cancelled by user or no device selected.', 'info');
    } else {
      log(`Bluetooth scan error: [${err.name}] ${err.message}`, 'error');
    }
  }
}

window.addEventListener('DOMContentLoaded', () => {
  checkEnvironment();

  document.getElementById('btn-scan')?.addEventListener('click', scanBluetooth);
  document.getElementById('btn-recheck')?.addEventListener('click', () => {
    log('Re-checking environment...', 'info');
    checkEnvironment();
  });
  document.getElementById('btn-clear-log')?.addEventListener('click', () => {
    const logContent = document.getElementById('log-content');
    if (logContent) logContent.innerHTML = '';
  });
  document.getElementById('btn-open-pixelbuds')?.addEventListener('click', () => {
    log('Navigating to mypixelbuds.google.com...', 'info');
    window.location.href = 'https://mypixelbuds.google.com';
  });
});
