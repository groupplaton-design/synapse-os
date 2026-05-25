// Windows-style layout: one full-width bottom panel.
// Start menu (Kickoff) bottom-left, icon task manager, tray + clock bottom-right.
// Plasma scripting API: https://develop.kde.org/docs/plasma/scripting/

for (var i = 0; i < panels().length; i++) { panels()[i].remove(); }

var panel = new Panel;
panel.location = "bottom";
panel.height = 44;

panel.addWidget("org.kde.plasma.kickoff");          // the Start menu
panel.addWidget("org.kde.plasma.icontasks");        // taskbar (icons-only)
panel.addWidget("org.kde.plasma.marginsseparator"); // push the rest right
panel.addWidget("org.kde.plasma.systemtray");
panel.addWidget("org.kde.plasma.digitalclock");
panel.addWidget("org.kde.plasma.showdesktop");      // "show desktop" far corner
