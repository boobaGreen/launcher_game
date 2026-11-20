param(
    [string]$TargetId = ""
)

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

$screens = [System.Windows.Forms.Screen]::AllScreens | Sort-Object { $_.Bounds.X }
$forms = @()
$idx = 1

foreach ($screen in $screens) {
    if (![string]::IsNullOrEmpty($TargetId) -and $TargetId -ne "all" -and $TargetId -ne "auto") {
        if ($screen.DeviceName -ne $TargetId) {
            $idx++
            continue
        }
    }

    $form = New-Object System.Windows.Forms.Form
    $form.FormBorderStyle = [System.Windows.Forms.FormBorderStyle]::None
    $form.StartPosition = [System.Windows.Forms.FormStartPosition]::Manual
    $cardW = 380
    $cardH = 220
    $form.Size = New-Object System.Drawing.Size($cardW, $cardH)
    $form.Location = New-Object System.Drawing.Point(
        [int]($screen.Bounds.X + ($screen.Bounds.Width - $cardW) / 2),
        [int]($screen.Bounds.Y + ($screen.Bounds.Height - $cardH) / 2)
    )
    $form.BackColor = [System.Drawing.Color]::FromArgb(15, 23, 42)
    $form.TopMost = $true
    $form.ShowInTaskbar = $false
    $form.Opacity = 0.94

    # Top Tag
    $tagLbl = New-Object System.Windows.Forms.Label
    $tagLbl.Text = "RETROHUB DISPLAY IDENTIFIER"
    $tagLbl.Font = [System.Drawing.Font]::new("Segoe UI", [float]9, [System.Drawing.FontStyle]::Bold)
    $tagLbl.ForeColor = [System.Drawing.Color]::FromArgb(148, 163, 184)
    $tagLbl.TextAlign = [System.Drawing.ContentAlignment]::MiddleCenter
    $tagLbl.Dock = [System.Windows.Forms.DockStyle]::Top
    $tagLbl.Height = 35
    $form.Controls.Add($tagLbl)

    # Large Number
    $numLbl = New-Object System.Windows.Forms.Label
    $numLbl.Text = "$idx"
    $numLbl.Font = [System.Drawing.Font]::new("Segoe UI", [float]56, [System.Drawing.FontStyle]::Bold)
    $numLbl.ForeColor = [System.Drawing.Color]::FromArgb(0, 229, 180)
    $numLbl.TextAlign = [System.Drawing.ContentAlignment]::MiddleCenter
    $numLbl.Dock = [System.Windows.Forms.DockStyle]::Top
    $numLbl.Height = 90
    $form.Controls.Add($numLbl)

    # Info Label
    $typeStr = if ($screen.Primary) { "Laptop / Principale" } else { "Monitor Esterno" }
    $infoLbl = New-Object System.Windows.Forms.Label
    $infoLbl.Text = "$typeStr`n$($screen.Bounds.Width) x $($screen.Bounds.Height)  [$($screen.DeviceName)]"
    $infoLbl.Font = [System.Drawing.Font]::new("Segoe UI", [float]11, [System.Drawing.FontStyle]::Bold)
    $infoLbl.ForeColor = [System.Drawing.Color]::FromArgb(241, 245, 249)
    $infoLbl.TextAlign = [System.Drawing.ContentAlignment]::MiddleCenter
    $infoLbl.Dock = [System.Windows.Forms.DockStyle]::Fill
    $form.Controls.Add($infoLbl)

    # Cyan border on paint
    $form.Add_Paint({
        param($sender, $e)
        $pen = [System.Drawing.Pen]::new([System.Drawing.Color]::FromArgb(0, 229, 180), 3)
        $e.Graphics.DrawRectangle($pen, 1, 1, $sender.Width - 3, $sender.Height - 3)
        $pen.Dispose()
    })

    $forms += $form
    $idx++
}

if ($forms.Count -eq 0) {
    exit 0
}

# Auto-close after 2200 ms
$timer = New-Object System.Windows.Forms.Timer
$timer.Interval = 2200
$timer.Add_Tick({
    $timer.Stop()
    foreach ($f in $forms) {
        $f.Close()
        $f.Dispose()
    }
    [System.Windows.Forms.Application]::ExitThread()
})
$timer.Start()

foreach ($f in $forms) {
    $f.Show()
}

[System.Windows.Forms.Application]::Run()
