<#
================================================================================
 ihotel-uia-harness.ps1 — THROWAWAY dev harness for the iHOTEL refresh action
================================================================================

 WHAT THIS IS
 ------------
 A self-contained fake iHOTEL. It stands up a WinForms window whose controls are
 NAMED exactly like the real ones — `ButtonX3` (Refresh) and `ButtonX6` (the
 "confirm multi-room selection" button) — so you can prove, on a scratch Windows
 box, that the middleware's Stage 4 refresh action works BEFORE anyone points it
 at a live reception terminal.

 It exists because two assumptions in
 `thai-id-middleware-tauri/src-tauri/src/ihotel/windows_impl.rs` cannot be tested
 on the macOS dev machine and are expensive to test wrong on a live terminal:

   1. UI Automation really does expose a WinForms control's `Name` as its
      `AutomationId`, so searching for "ButtonX3" finds the button.
   2. A posted WM_LBUTTONDOWN/WM_LBUTTONUP pair really does raise `Click` on a
      control that derives from `Control` (NOT from `ButtonBase`) — which is what
      DevComponents' `ButtonX` is, and why `BM_CLICK` is not an option.

 `ButtonX3` here is therefore a hand-rolled `Control` subclass, deliberately
 mirroring `ButtonX`'s inheritance rather than being a stock `Button`. A green
 result against a stock Button would have proved nothing about the real target.

 Read `docs/legacy-app/ROOM_GRID_REFRESH.md` §5 and
 `docs/adr/0006-legacy-stale-notification.md` for what is being modelled.
 This script is scaffolding, not a product: delete it once the action is
 field-verified, or keep it for the next vendor upgrade. It never touches a
 database, a network, or anything outside its own window.

 REQUIREMENTS
 ------------
 * Windows PowerShell 5.1 (`powershell.exe`). NOT PowerShell 7 (`pwsh.exe`) —
   WinForms `Add-Type` compilation against the in-box assemblies is what 5.1
   gives you for free.
 * A built `thai-id-middleware.exe`, running.

 HOW TO RUN
 ----------
 1) Start the harness. `-WindowStyle Hidden` hides the PowerShell CONSOLE window
    so that the only visible, unowned top-level window of the process is the
    harness form itself — which is what the middleware looks for:

        powershell.exe -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden `
            -File scripts\dev\ihotel-uia-harness.ps1

    Close any OTHER PowerShell windows first: the middleware matches on process
    name, and a stray console would be an equally valid match.

 2) Start the middleware pointed at the harness instead of at iHOTEL. All three
    targets are env-overridable exactly so this is config, not a rebuild:

        $env:IHOTEL_PROCESS_NAME     = "powershell.exe"   # instead of HOTEL.exe
        $env:IHOTEL_REFRESH_BUTTON_ID = "ButtonX3"        # (default; shown for clarity)
        $env:IHOTEL_SELECTION_BUTTON_ID = "ButtonX6"      # (default; shown for clarity)
        .\thai-id-middleware.exe

    The middleware logs the resolved targets once at startup — check that line
    first if anything below misbehaves.

 3) Drive it over HTTP from a THIRD PowerShell window:

        # Should report processFound=true, gridWindowFound=true
        Invoke-RestMethod http://127.0.0.1:9898/ihotel/status

        # Should report sent=true, and the harness counter should tick up by 1
        Invoke-RestMethod -Method Post http://127.0.0.1:9898/ihotel/refresh

 THE TEST MATRIX — run all six, in this order
 --------------------------------------------
 Each row maps to one guard in `ihotel::decide`. Expected values are what
 `POST /ihotel/refresh` should answer.

   A. HAPPY PATH
      Harness open, ButtonX6 hidden, no modal.
      -> {"sent": true}; counter +1; ButtonX6 stays hidden.

   B. SELECTION PENDING  (the most important guard)
      Tick "Show ButtonX6 (selection pending)".
      -> {"sent": false, "reason": "selection-pending"}; counter UNCHANGED.
      This is the guard that protects reception's in-progress multi-room
      selection from being wiped by `ClearCheck()`. If it ever regresses, the
      damage is silent and happens on a screen she isn't looking at.

   C. MODAL OPEN
      Click "Open modal dialog" and leave the dialog up.
      -> {"sent": false, "reason": "modal-open"}; counter UNCHANGED.
      Close the dialog and re-run: back to {"sent": true}.

   D. GRID NOT FOUND
      Tick "Hide ButtonX3 (grid closed)".
      -> {"sent": false, "reason": "grid-not-found"}.
      (Don't close the form to test this — the script exits with it, which
      gets you row E instead.)

   E. PROCESS NOT FOUND
      Close the harness form, which ends the script and the process. Make
      sure no other powershell.exe is running.
      -> {"sent": false, "reason": "process-not-found"}.

   F. OBSCURED / UNFOCUSED  (the assumption most likely to be wrong)
      Restart the harness for a clean state. Now COVER the harness window
      completely with another window (a maximised Notepad works) and click into
      that other window so the harness is neither focused NOR visible on screen.
      Then POST /ihotel/refresh, and afterwards uncover the harness and read the
      counter.
      -> {"sent": true} AND the counter MUST have incremented.

      Why this row is load-bearing, and why it may well FAIL: the entire
      feature exists because iHOTEL's own refresh timer is dead while iHOTEL is
      unfocused. But WinForms' `Control.WmMouseUp` raises `Click` only when
      `ControlStyles.StandardClick` AND `STATE_MOUSEPRESSED` AND `!IsDisposed`
      AND `WindowFromPoint(PointToScreen(lParam)) == Handle` all hold —
      verified verbatim against Microsoft's published reference source for
      .NET Framework `Control.cs` and against current dotnet/winforms. That
      last clause asks what is ACTUALLY VISIBLE at that screen point, so an
      obscured control gets MouseDown and MouseUp but NOT Click.

      This row exists to find out whether that gate is what governs the real
      `ButtonX3`. It may not: DevComponents controls commonly raise `Click`
      from their own `OnMouseUp` override, which runs unconditionally. If row
      F fails HERE (against a plain `Control` subclass) that is the expected
      standard-click behaviour, not a bug in the middleware — the question
      that actually matters is whether it fails on the REAL terminal. Run it
      there too, and report the result before anyone relies on this feature.

      If row F fails on a real terminal, the middleware needs a different
      lever (UI Automation's Invoke / LegacyIAccessible DoDefaultAction are
      the leads) — that is a design decision, not a patch.

      The counter is authoritative; the HTTP response only says the messages
      were POSTED, never that iHOTEL acted on them.

 WHAT "SUCCESS" DOES NOT PROVE
 -----------------------------
 A green run here proves the LOCATE + POST mechanics against a control with the
 right base class. It does NOT prove that DevComponents' real `ButtonX` reacts
 identically, nor that `LoadRooms` does what we expect. Those need a real
 terminal, coordinated with reception.
================================================================================
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

# A `Control`-derived button, mirroring DevComponents' `ButtonX` inheritance.
# It is NOT a `Button`/`ButtonBase` on purpose: that difference is precisely why
# the middleware posts WM_LBUTTONDOWN/UP instead of BM_CLICK, so the harness has
# to reproduce it or it validates the wrong thing.
Add-Type -ReferencedAssemblies System.Windows.Forms, System.Drawing -TypeDefinition @'
using System;
using System.Drawing;
using System.Windows.Forms;

public class ButtonXLookalike : Control
{
    public ButtonXLookalike()
    {
        SetStyle(ControlStyles.StandardClick
               | ControlStyles.UserPaint
               | ControlStyles.AllPaintingInWmPaint
               | ControlStyles.OptimizedDoubleBuffer, true);
        BackColor = Color.FromArgb(226, 232, 240);
        Size = new Size(160, 40);
    }

    protected override void OnPaint(PaintEventArgs e)
    {
        e.Graphics.Clear(BackColor);
        ControlPaint.DrawBorder3D(e.Graphics, ClientRectangle, Border3DStyle.Raised);
        TextRenderer.DrawText(e.Graphics, Text, Font, ClientRectangle, ForeColor,
            TextFormatFlags.HorizontalCenter | TextFormatFlags.VerticalCenter);
    }
}
'@

$form               = New-Object System.Windows.Forms.Form
$form.Name          = 'FormRoomMainFake'
$form.Text          = 'iHOTEL UIA harness (fake FormRoomMain)'
$form.ClientSize    = New-Object System.Drawing.Size(560, 300)
$form.StartPosition = 'CenterScreen'
$form.TopMost       = $false   # must be able to sit BEHIND other windows for row F

$header = New-Object System.Windows.Forms.Label
$header.AutoSize = $true
$header.Location = New-Object System.Drawing.Point(16, 12)
$header.MaximumSize = New-Object System.Drawing.Size(520, 0)
$header.Text = @'
Fake iHOTEL room grid. Controls are NAMED like the real ones so UI Automation
sees the same AutomationIds. Nothing here touches a database.
'@

# --- ButtonX3: the Refresh lever -------------------------------------------
# Real handler (FormRoomMain.cs:5743) is:
#     DateTimePicker1.Value = DateTime.Now;  // -> LoadRooms
#     ClearCheck();                          // -> ButtonX6.Visible = false
# We model the observable half: bump a counter, and hide ButtonX6.
$buttonX3          = New-Object ButtonXLookalike
$buttonX3.Name     = 'ButtonX3'      # -> UIA AutomationId
$buttonX3.Text     = 'Refresh'
$buttonX3.Location = New-Object System.Drawing.Point(16, 80)

# --- ButtonX6: "confirm multi-room selection" -------------------------------
# Visible == a selection is in flight == the middleware must refuse to refresh.
$buttonX6           = New-Object System.Windows.Forms.Button
$buttonX6.Name      = 'ButtonX6'     # -> UIA AutomationId
$buttonX6.Text      = 'ยืนยันเลือกห้อง'
$buttonX6.Size      = New-Object System.Drawing.Size(160, 40)
$buttonX6.Location  = New-Object System.Drawing.Point(200, 80)
$buttonX6.Visible   = $false         # start hidden: no selection pending

$counter = 0

$counterLabel          = New-Object System.Windows.Forms.Label
$counterLabel.AutoSize = $true
$counterLabel.Font     = New-Object System.Drawing.Font('Segoe UI', 16, [System.Drawing.FontStyle]::Bold)
$counterLabel.Location = New-Object System.Drawing.Point(16, 140)
$counterLabel.Text     = 'ButtonX3 clicks: 0'

$lastLabel          = New-Object System.Windows.Forms.Label
$lastLabel.AutoSize = $true
$lastLabel.Location = New-Object System.Drawing.Point(16, 176)
$lastLabel.Text     = 'last click: (none)'

$showSelection          = New-Object System.Windows.Forms.CheckBox
$showSelection.AutoSize = $true
$showSelection.Location = New-Object System.Drawing.Point(16, 206)
$showSelection.Text     = 'Show ButtonX6 (selection pending)  -> expect reason=selection-pending'

$hideRefresh          = New-Object System.Windows.Forms.CheckBox
$hideRefresh.AutoSize = $true
$hideRefresh.Location = New-Object System.Drawing.Point(16, 230)
$hideRefresh.Text     = 'Hide ButtonX3 (grid closed)  -> expect reason=grid-not-found'

$modalButton          = New-Object System.Windows.Forms.Button
$modalButton.Name     = 'btnOpenModal'
$modalButton.Text     = 'Open modal dialog'
$modalButton.Size     = New-Object System.Drawing.Size(160, 32)
$modalButton.Location = New-Object System.Drawing.Point(384, 206)

# Counting Click (not MouseUp) is deliberate: Click is the event iHOTEL's real
# handler is wired to, so it is the only signal that proves the posted message
# pair reached the destination that matters.
$buttonX3.Add_Click({
    $script:counter++
    $counterLabel.Text = "ButtonX3 clicks: $script:counter"
    $lastLabel.Text    = "last click: $(Get-Date -Format 'HH:mm:ss.fff')"
    # Mirror ClearCheck(): a refresh destroys the pending selection.
    $buttonX6.Visible    = $false
    $showSelection.Checked = $false
})

$showSelection.Add_CheckedChanged({ $buttonX6.Visible = $showSelection.Checked })
$hideRefresh.Add_CheckedChanged({ $buttonX3.Visible = -not $hideRefresh.Checked })

# ShowDialog() disables the owner form, which is exactly the state the
# `modal-open` guard reads via IsWindowEnabled on the top-level shell window.
$modalButton.Add_Click({
    $dialog             = New-Object System.Windows.Forms.Form
    $dialog.Text        = 'Modal (owner is disabled while this is up)'
    $dialog.ClientSize  = New-Object System.Drawing.Size(360, 120)
    $dialog.StartPosition = 'CenterParent'
    $dialogLabel          = New-Object System.Windows.Forms.Label
    $dialogLabel.AutoSize = $true
    $dialogLabel.Location = New-Object System.Drawing.Point(16, 20)
    $dialogLabel.Text     = "POST /ihotel/refresh now.`nExpect reason=modal-open, counter unchanged."
    $closeButton          = New-Object System.Windows.Forms.Button
    $closeButton.Text     = 'Close'
    $closeButton.Location = New-Object System.Drawing.Point(16, 70)
    $closeButton.Add_Click({ $dialog.Close() })
    $dialog.Controls.AddRange(@($dialogLabel, $closeButton))
    [void]$dialog.ShowDialog($form)
    $dialog.Dispose()
})

$form.Controls.AddRange(@(
    $header, $buttonX3, $buttonX6, $counterLabel, $lastLabel,
    $showSelection, $hideRefresh, $modalButton
))

[void]$form.ShowDialog()
$form.Dispose()
