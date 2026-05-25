// GNOME / Ubuntu-style layout: top bar + left vertical dock.
// Top: launcher/"activities" left, clock centre, tray right.
// Left: vertical icon dock (dash-to-dock feel).

for (var i = 0; i < panels().length; i++) { panels()[i].remove(); }

// --- top bar ---
var top = new Panel;
top.location = "top";
top.height = 30;
top.addWidget("org.kde.plasma.kickoff");      // "Activities"-like launcher, left
top.addWidget("org.kde.plasma.panelspacer");
top.addWidget("org.kde.plasma.digitalclock"); // clock centre-ish
top.addWidget("org.kde.plasma.panelspacer");
top.addWidget("org.kde.plasma.systemtray");

// --- left vertical dock ---
var dock = new Panel;
dock.location = "left";
dock.height = 56;                 // = width for a vertical panel
dock.hiding = "dodgewindows";
dock.addWidget("org.kde.plasma.icontasks");

// NOTE: GNOME's overview maps to KWin's "Overview" effect (Meta / hot-corner);
// wire that in apply-preset.sh once validated on a boot test.
