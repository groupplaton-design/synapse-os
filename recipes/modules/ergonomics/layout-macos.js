// macOS-style layout: thin top menu bar + bottom Dock.
// Top: global menu (app menus) left, tray + clock right.
// Bottom: auto-hiding, centred icon Dock.

for (var i = 0; i < panels().length; i++) { panels()[i].remove(); }

// --- top menu bar ---
var top = new Panel;
top.location = "top";
top.height = 28;
top.addWidget("org.kde.plasma.kickoff");      // app launcher at the very left
top.addWidget("org.kde.plasma.appmenu");      // global menu (active app's menus)
top.addWidget("org.kde.plasma.panelspacer");  // push status items to the right
top.addWidget("org.kde.plasma.systemtray");
top.addWidget("org.kde.plasma.digitalclock");

// --- bottom Dock ---
var dock = new Panel;
dock.location = "bottom";
dock.height = 56;
dock.hiding = "autohide";        // dodge/auto-hide like the macOS Dock
dock.addWidget("org.kde.plasma.icontasks");   // pinned + running apps, icons only

// NOTE: true centring + floating rounded dock needs per-version tuning
// (panel.alignment / minimumLength) — verify on a boot test.
