; Nebula Installer - Maximum Effort Edition
; Requires Inno Setup 6.0+

#define AppName "Nebula"
#define AppVersion "1.0.0"
#define AppPublisher "Nebula Contributors"
#define AppCopyright "Copyright (c) 2026 Nebula"
#define AppURL "https://nebula-lang.org"
#define ExtVersion "1.0.0"

[Setup]
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
AppCopyright={#AppCopyright}
AppSupportURL={#AppURL}
AppUpdatesURL={#AppURL}
DefaultDirName={userappdata}\Nebula
DefaultGroupName=Nebula
DisableProgramGroupPage=yes
DisableWelcomePage=yes
DisableDirPage=yes
DisableReadyPage=yes
DisableFinishedPage=yes
Compression=lzma2/ultra64
SolidCompression=yes
PrivilegesRequired=lowest
OutputBaseFilename=nebula-install
OutputDir=.
WizardStyle=modern
WizardSizePercent=100
WindowResizable=no
MinVersion=10.0
UninstallDisplayIcon={app}\bin\nebula.exe
BackColor=$1E1E1E
BackColor2=$1E1E1E
WizardImageAlphaFormat=defined

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
; Core binary
Source: "..\target\release\nebula.exe"; DestDir: "{app}\bin"; Flags: ignoreversion skipifsourcedoesntexist
; Stdlib
Source: "..\lib\std\*"; DestDir: "{app}\lib\std"; Flags: ignoreversion recursesubdirs createallsubdirs skipifsourcedoesntexist
; License and README
Source: "..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist
; UI Resources
Source: "logo.bmp"; DestDir: "{tmp}"; Flags: dontcopy

[Tasks]
Name: "addtopath"; Description: "Add Nebula to PATH"; GroupDescription: "Configuration:"
Name: "desktopicon"; Description: "Create desktop shortcut"; GroupDescription: "Configuration:"; Flags: unchecked
Name: "fileassoc"; Description: "Associate .na files with Nebula"; GroupDescription: "Configuration:"; Flags: unchecked
Name: "vscode"; Description: "Install VS Code extension"; GroupDescription: "IDE Extensions:"; Check: DetectVSCode
Name: "neovim"; Description: "Install Neovim support"; GroupDescription: "IDE Extensions:"; Check: DetectNeovim
Name: "jetbrains"; Description: "Install JetBrains plugin"; GroupDescription: "IDE Extensions:"; Check: DetectJetBrains

[Icons]
Name: "{userdesktop}\Nebula"; Filename: "{app}\bin\nebula.exe"; Tasks: desktopicon

[Registry]
; Main application registry
Root: HKCU; Subkey: "Software\Nebula"; ValueType: string; ValueName: "InstallPath"; ValueData: "{app}"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Nebula"; ValueType: string; ValueName: "Version"; ValueData: "{#AppVersion}"
Root: HKCU; Subkey: "Software\Nebula"; ValueType: string; ValueName: "ExtensionVersion"; ValueData: "{#ExtVersion}"
Root: HKCU; Subkey: "Software\Nebula"; ValueType: string; ValueName: "InstallDate"; ValueData: "{code:GetInstallDate}"
; File association
Root: HKCU; Subkey: "Software\Classes\.na"; ValueType: string; ValueData: "NebulaScript"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\NebulaScript"; ValueType: string; ValueData: "Nebula Script File"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\NebulaScript\DefaultIcon"; ValueType: string; ValueData: "{app}\bin\nebula.exe,0"; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\NebulaScript\shell\open\command"; ValueType: string; ValueData: """{app}\bin\nebula.exe"" ""%1"""; Tasks: fileassoc

[Run]
; VS Code Extension Install
Filename: "{code:GetVSCodePath}"; Parameters: "--install-extension nebula-lang.nebula --force"; Flags: runhidden nowait skipifsilent; Tasks: vscode; Check: IsVSCodeDetected

[UninstallDelete]
Type: files; Name: "{app}\bin\nebula.exe"
Type: files; Name: "{app}\install.log"
Type: dirifempty; Name: "{app}\bin"
Type: dirifempty; Name: "{app}\lib\std"
Type: dirifempty; Name: "{app}\lib"
Type: dirifempty; Name: "{app}"

[Code]
// ============================================================================
// WINDOWS API IMPORTS (The Magic)
// ============================================================================
// function SetTimer(hWnd: longword; nIDEvent, uElapse: longword; lpTimerFunc: longword): longword;
//   external 'SetTimer@user32.dll stdcall';
// function KillTimer(hWnd: longword; nIDEvent: longword): longword;
//   external 'KillTimer@user32.dll stdcall';
function ReleaseCapture(): LongInt;
  external 'ReleaseCapture@user32.dll stdcall';
function SendMessage(hWnd: HWND; Msg: UINT; wParam: LongInt; lParam: LongInt): LongInt;
  external 'SendMessageA@user32.dll stdcall';
function CreateRoundRectRgn(x1, y1, x2, y2, w, h: Integer): LongWord;
  external 'CreateRoundRectRgn@gdi32.dll stdcall';
function SetWindowRgn(hWnd: HWND; hRgn: LongWord; bRedraw: Boolean): Integer;
  external 'SetWindowRgn@user32.dll stdcall';

// ============================================================================
// CONSTANTS & CONFIG
// ============================================================================
const
  // Window
  WIZ_WIDTH     = 600;
  WIZ_HEIGHT    = 450;
  CORNER_RADIUS = 16;
  
  // Dragging
  WM_SYSCOMMAND = $0112;
  HTCAPTION     = $0002;
  
  // Colors (BGR format for Inno Setup)
  // Purple-Blue Nebula Theme
  C_BG_TOP      = $4B1A0A;   // Deep Blue-Purple (RGB: 10, 26, 75)
  C_BG_BOT      = $0A0005;   // Near Black (RGB: 5, 0, 10)
  C_GLASS       = $503020;   // Glassy panel (semi-dark purple-blue)
  C_GLASS_BORDER= $8B6FF0;   // Nebula Purple border
  C_PRIMARY     = $F06F8B;   // Nebula Purple (BGR for #8B6FF0)
  C_PRIMARY_HOV = $FF85A0;   // Lighter Purple
  C_ACCENT      = $B8E600;   // Electric Teal
  C_TEXT        = $FFFFFF;
  C_TEXT_DIM    = $B0B0B0;

  // Stars
  MAX_STARS     = 120;       // More stars!
  LOGO_Y_START  = 50;

// ============================================================================
// TYPES
// ============================================================================
type
  TStar = record
    X, Y: Double;
    Speed: Double;
    Size: Integer; // 1=Dot, 2=Shooting
    Trail: Double;
    Brightness: Integer; // 0-255
  end;

  TButtonInfo = record
    Panel: TPanel;
    Label_: TLabel;
    IsPrimary: Boolean;
    IsClose: Boolean;
    OnClick: TNotifyEvent;
  end;

// ============================================================================
// GLOBALS
// ============================================================================
var
  // Graphics
  RenderBitmap: TBitmap;
  BackgroundImg: TBitmapImage;
  LogoImg: TBitmapImage;
  Stars: array [0..MAX_STARS] of TStar;
  
  // Animation State
  LogoAlpha: Integer;
  LogoParam: Double;
  
  // Window
  Buttons: array of TButtonInfo;
  
  // Pages
  PageWelcome: TWizardPage;
  PageLicense: TWizardPage;
  PageDir: TWizardPage;
  PageComps: TWizardPage;
  PageProgress: TWizardPage;
  PageFinish: TWizardPage;
  
  // Data
  VSCodeFound: Boolean;
  VSCodeBin: String;
  NeovimFound: Boolean;
  JetBrainsFound: Boolean;

// ============================================================================
// PARTICLE SYSTEM
// ============================================================================
procedure InitParticles;
var
  I: Integer;
begin
  for I := 0 to MAX_STARS do
  begin
    Stars[I].X := Random(WIZ_WIDTH + 100);
    Stars[I].Y := Random(WIZ_HEIGHT);
    
    // 60% background dots, 40% shooting stars (more visible!)
    if Random(10) < 6 then
    begin
      Stars[I].Speed := 0;
      Stars[I].Size := 1;
      Stars[I].Trail := 0;
      Stars[I].Brightness := 80 + Random(175);
    end
    else
    begin
      Stars[I].Speed := 2.0 + Random(6);
      Stars[I].Size := 2;
      Stars[I].Trail := 20.0 + Random(50); // LONGER TRAILS
      Stars[I].Brightness := 150 + Random(105);
    end;
  end;
  LogoAlpha := 255;
  LogoParam := 1.0;
end;

procedure UpdateParticles;
begin
  // Static render - no update needed
end;

function RGB(R, G, B: Byte): TColor;
begin
  Result := (R) or (G shl 8) or (B shl 16);
end;

procedure RenderFrame;
var
  CV: TCanvas;
  I, Y: Integer;
  R1, G1, B1: Integer; // Top color components
  R2, G2, B2: Integer; // Bottom color components
  R, G, B: Integer;
  Col: TColor;
  StarX, StarY, TrailX, TrailY: Integer;
begin
  CV := RenderBitmap.Canvas;
  
  // =========================================================================
  // 1. GRADIENT: Deep Purple-Blue (Top) to Near-Black (Bottom)
  // =========================================================================
  // Top:    RGB(20, 10, 60) - Deep indigo/purple
  // Bottom: RGB(5, 2, 15)   - Near black with purple tint
  R1 := 20;  G1 := 10;  B1 := 60;
  R2 := 5;   G2 := 2;   B2 := 15;
  
  for Y := 0 to WIZ_HEIGHT - 1 do
  begin
    R := R1 + MulDiv(R2 - R1, Y, WIZ_HEIGHT);
    G := G1 + MulDiv(G2 - G1, Y, WIZ_HEIGHT);
    B := B1 + MulDiv(B2 - B1, Y, WIZ_HEIGHT);
    
    CV.Pen.Color := RGB(R, G, B);
    CV.MoveTo(0, Y);
    CV.LineTo(WIZ_WIDTH, Y);
  end;
  
  // =========================================================================
  // 2. STARS: Dots and Shooting Stars with trails
  // =========================================================================
  for I := 0 to MAX_STARS do
  begin
    // Color based on brightness (white/blue tint)
    if Stars[I].Brightness > 220 then
      Col := RGB(255, 255, 255) // Bright white
    else if Stars[I].Brightness > 180 then
      Col := RGB(200, 210, 255) // Blue-white
    else if Stars[I].Brightness > 120 then
      Col := RGB(150, 160, 200) // Dim blue
    else
      Col := RGB(80, 90, 130);  // Very dim
    
    StarX := Trunc(Stars[I].X);
    StarY := Trunc(Stars[I].Y);
    
    if Stars[I].Size = 1 then
    begin
      // Static dot
      if (StarX >= 0) and (StarX < WIZ_WIDTH) and (StarY >= 0) and (StarY < WIZ_HEIGHT) then
        CV.Pixels[StarX, StarY] := Col;
    end
    else
    begin
      // Shooting star with trail
      TrailX := Trunc(Stars[I].X - Stars[I].Trail);
      TrailY := Trunc(Stars[I].Y - (Stars[I].Trail * 0.5));
      
      // Draw thick trail (2px wide for visibility)
      CV.Pen.Color := Col;
      CV.Pen.Width := 2;
      CV.MoveTo(StarX, StarY);
      CV.LineTo(TrailX, TrailY);
      CV.Pen.Width := 1;
      
      // Bright head
      if (StarX >= 1) and (StarX < WIZ_WIDTH-1) and (StarY >= 1) and (StarY < WIZ_HEIGHT-1) then
      begin
        CV.Pixels[StarX, StarY] := clWhite;
        CV.Pixels[StarX-1, StarY] := clWhite;
        CV.Pixels[StarX, StarY-1] := clWhite;
      end;
    end;
  end;
  
  // =========================================================================
  // 3. COMMIT TO UI
  // =========================================================================
  BackgroundImg.Bitmap := RenderBitmap;
  
  if LogoImg <> nil then
    LogoImg.Visible := True;
end;

procedure OnTimer(Sender: TObject);
begin
  // No-op
end;

// ============================================================================
// DRAGGABLE WINDOW LOGIC
// ============================================================================
procedure OnMouseDown(Sender: TObject; Button: TMouseButton; Shift: TShiftState; X, Y: Integer);
begin
  if Button = mbLeft then
  begin
    ReleaseCapture;
    SendMessage(WizardForm.Handle, WM_SYSCOMMAND, $F012, 0);
  end;
end;

// ============================================================================
// CUSTOM BUTTONS
// ============================================================================
procedure BtnHandler(Sender: TObject);
var
  I: Integer;
begin
  for I := 0 to GetArrayLength(Buttons) - 1 do
  begin
    if (Buttons[I].Panel = Sender) or (Buttons[I].Label_ = Sender) then
    begin
      Buttons[I].OnClick(Sender);
      Break;
    end;
  end;
end;

procedure AddButton(Parent: TWinControl; Text: String; IsPri, IsCls: Boolean; X, Y, W, H: Integer; Event: TNotifyEvent);
var
  Idx: Integer;
  P: TPanel;
  L: TLabel;
begin
  Idx := GetArrayLength(Buttons);
  SetArrayLength(Buttons, Idx + 1);
  
  P := TPanel.Create(Parent);
  P.Parent := Parent;
  P.SetBounds(X, Y, W, H);
  P.BevelOuter := bvNone;
  
  // Color scheme: Primary=Purple, Secondary=Glassy, Close=Transparent
  if IsCls then 
    P.Color := $1A0A10  // Very dark, almost transparent
  else if IsPri then 
    P.Color := C_PRIMARY  // Nebula Purple
  else 
    P.Color := C_GLASS;   // Glassy blue-purple
  
  P.OnClick := @BtnHandler;
  P.Cursor := crHand;
  P.ParentBackground := False;
  
  L := TLabel.Create(P);
  L.Parent := P;
  L.Caption := Text;
  L.Font.Name := 'Segoe UI';
  if IsCls then
  begin
    L.Font.Size := 10;
    L.Font.Style := [fsBold];
    L.Font.Color := $666666;
  end
  else
  begin
    L.Font.Size := 9;
    L.Font.Style := [fsBold];
    L.Font.Color := C_TEXT;
  end;
  
  L.AutoSize := True;
  L.Left := (W - L.Width) div 2;
  L.Top := (H - L.Height) div 2;
  L.OnClick := @BtnHandler;
  L.Cursor := crHand;
  L.Transparent := True; // Ensure label takes panel color
  
  Buttons[Idx].Panel := P;
  Buttons[Idx].Label_ := L;
  Buttons[Idx].IsPrimary := IsPri;
  Buttons[Idx].IsClose := IsCls;
  Buttons[Idx].OnClick := Event;
end;

// ============================================================================
// ACTIONS
// ============================================================================
procedure ActClose(Sender: TObject); begin WizardForm.Close; end;
procedure ActMin(Sender: TObject); begin SendMessage(WizardForm.Handle, WM_SYSCOMMAND, $F020, 0); end;
procedure ActNext(Sender: TObject); begin WizardForm.NextButton.OnClick(WizardForm.NextButton); end;
procedure ActBack(Sender: TObject); begin WizardForm.BackButton.OnClick(WizardForm.BackButton); end;

// ============================================================================
// IDE LOGIC
// ============================================================================
function CheckFile(Path: String): Boolean;
begin
  Result := FileExists(ExpandConstant(Path));
end;

procedure CheckIDEs;
begin
  VSCodeFound := CheckFile('{localappdata}\Programs\Microsoft VS Code\bin\code.cmd') or 
                 CheckFile('C:\Program Files\Microsoft VS Code\bin\code.cmd');
  if VSCodeFound then 
     if CheckFile('{localappdata}\Programs\Microsoft VS Code\bin\code.cmd') then
        VSCodeBin := ExpandConstant('{localappdata}\Programs\Microsoft VS Code\bin\code.cmd')
     else
        VSCodeBin := 'C:\Program Files\Microsoft VS Code\bin\code.cmd';
        
  NeovimFound := CheckFile('{localappdata}\nvim\bin\nvim.exe');
  JetBrainsFound := DirExists(ExpandConstant('{localappdata}\JetBrains\Toolbox'));
end;

function DetectVSCode: Boolean; begin Result := VSCodeFound; end;
function DetectNeovim: Boolean; begin Result := NeovimFound; end;
function DetectJetBrains: Boolean; begin Result := JetBrainsFound; end;
function IsVSCodeDetected: Boolean; begin Result := VSCodeFound; end;
function GetVSCodePath(Param: String): String; begin Result := VSCodeBin; end;
function GetInstallDate(Param: String): String; begin Result := GetDateTimeString('yyyy-mm-dd', '-', ':'); end;

// ============================================================================
// PAGES
// ============================================================================
procedure BuildUI;
var
  LogoFn: String;
begin
  // --- WELCOME ---
  PageWelcome := CreateCustomPage(wpWelcome, '', '');
  
  // Logo
  LogoFn := ExpandConstant('{tmp}\logo.bmp');
  ExtractTemporaryFile('logo.bmp');
  LogoImg := TBitmapImage.Create(PageWelcome);
  LogoImg.Parent := PageWelcome.Surface;
  LogoImg.Bitmap.LoadFromFile(LogoFn);
  LogoImg.Width := 80; LogoImg.Height := 80;
  LogoImg.Stretch := True;
  LogoImg.Left := (WIZ_WIDTH - 80) div 2;
  LogoImg.Top := 80;
  
  // Title
  with TLabel.Create(PageWelcome) do begin
    Parent := PageWelcome.Surface;
    Caption := 'Nebula';
    Font.Size := 28; Font.Style := [fsBold]; Font.Color := C_TEXT; Font.Name := 'Segoe UI';
    AutoSize := True;
    Left := (WIZ_WIDTH - Width) div 2; Top := 170;
    Transparent := True;
  end;
  
  with TLabel.Create(PageWelcome) do begin
    Parent := PageWelcome.Surface;
    Caption := 'Logic is Electric.';
    Font.Size := 12; Font.Color := C_PRIMARY; Font.Name := 'Segoe UI';
    AutoSize := True;
    Left := (WIZ_WIDTH - Width) div 2; Top := 220;
    Transparent := True;
  end;
  
  AddButton(PageWelcome.Surface, 'Install Nebula', True, False, (WIZ_WIDTH - 160) div 2, 320, 160, 40, @ActNext);

  // --- LICENSE ---
  PageLicense := CreateCustomPage(PageWelcome.ID, '', '');
  with TLabel.Create(PageLicense) do begin
    Parent := PageLicense.Surface; Caption := 'License Agreement'; Font.Size := 16; Font.Color := C_TEXT;
    Left := 30; Top := 30; Transparent := True;
  end;
  
  with TNewMemo.Create(PageLicense) do begin
    Parent := PageLicense.Surface; SetBounds(30, 70, WIZ_WIDTH-60, 200);
    Color := C_GLASS; Font.Color := C_TEXT;
    Lines.Text := 'MIT License'#13#10'Copyright (c) 2026 Nebula'#13#10#13#10'Permission is hereby granted...';
    ReadOnly := True; ScrollBars := ssVertical;
  end;
  
  with TCheckBox.Create(PageLicense) do begin
    Parent := PageLicense.Surface; Caption := 'I accept the license terms';
    Left := 30; Top := 290; Width := 300; Font.Color := C_TEXT;
    Checked := True;
    // Transparent doesn't work well on checkbox in Inno, depends on theme
  end;
  
  AddButton(PageLicense.Surface, 'Back', False, False, 30, 380, 100, 36, @ActBack);
  AddButton(PageLicense.Surface, 'Next', True, False, WIZ_WIDTH-130, 380, 100, 36, @ActNext);
  
  // --- DIRECTORY ---
  PageDir := CreateCustomPage(PageLicense.ID, '', '');
  with TLabel.Create(PageDir) do begin
    Parent := PageDir.Surface; Caption := 'Installation Location'; Font.Size := 16; Font.Color := C_TEXT;
    Left := 30; Top := 30; Transparent := True;
  end;
  
  with TEdit.Create(PageDir) do begin
    Parent := PageDir.Surface; SetBounds(30, 80, WIZ_WIDTH-60, 30);
    Text := ExpandConstant('{userappdata}\Nebula');
    Color := C_GLASS; Font.Color := C_TEXT; Font.Size := 10;
    BorderStyle := bsNone; // Flat
  end;
  // Tasks
  with TCheckBox.Create(PageDir) do begin
    Parent := PageDir.Surface; Caption := 'Add Nebula to PATH'; Checked := True;
    Left := 30; Top := 140; Width := 300; Font.Color := C_TEXT;
  end;
  with TCheckBox.Create(PageDir) do begin
    Parent := PageDir.Surface; Caption := 'Create Desktop Shortcut'; Checked := True;
    Left := 30; Top := 170; Width := 300; Font.Color := C_TEXT;
  end;
  
  AddButton(PageDir.Surface, 'Back', False, False, 30, 380, 100, 36, @ActBack);
  AddButton(PageDir.Surface, 'Install', True, False, WIZ_WIDTH-130, 380, 100, 36, @ActNext);
  
  // --- PROGRESS ---
  PageProgress := CreateCustomPage(PageDir.ID, '', '');
  with TLabel.Create(PageProgress) do begin
    Parent := PageProgress.Surface; Caption := 'Installing Nebula...'; Font.Size := 16; Font.Color := C_TEXT;
    Left := 30; Top := 150; Transparent := True;
  end;
  
  with TNewProgressBar.Create(PageProgress) do begin
    Parent := PageProgress.Surface; SetBounds(30, 200, WIZ_WIDTH-60, 6);
    // Styling progress bar is hard, standard green usually
  end;
  
  // --- FINISH ---
  PageFinish := CreateCustomPage(PageProgress.ID, '', '');
  with TLabel.Create(PageFinish) do begin
    Parent := PageFinish.Surface; Caption := 'Installation Complete'; Font.Size := 20; Font.Color := C_PRIMARY;
    Left := (WIZ_WIDTH-Width) div 2; Top := 150; Transparent := True;
  end;
  AddButton(PageFinish.Surface, 'Exit', True, False, (WIZ_WIDTH-120) div 2, 300, 120, 36, @ActClose);
end;

// ============================================================================
// MAIN SETUP
// ============================================================================
procedure InitializeWizard;
var
  Rgn: LongWord;
begin
  // 1. Setup Form
  WizardForm.BorderStyle := bsNone;
  WizardForm.ClientWidth := WIZ_WIDTH;
  WizardForm.ClientHeight := WIZ_HEIGHT;
  WizardForm.Color := C_BG_BOT; // This will be covered by BackgroundImg
  WizardForm.InnerNotebook.Visible := True; // Need this for pages
  WizardForm.OuterNotebook.Visible := False; // Hide top chrome
  WizardForm.Bevel.Visible := False;
  WizardForm.MainPanel.Visible := False;

  // IMPORTANT: Hide standard buttons initially
  WizardForm.NextButton.Visible := False;
  WizardForm.BackButton.Visible := False;
  WizardForm.CancelButton.Visible := False;
  
  // Round Corners
  Rgn := CreateRoundRectRgn(0, 0, WIZ_WIDTH, WIZ_HEIGHT, CORNER_RADIUS, CORNER_RADIUS);
  SetWindowRgn(WizardForm.Handle, Rgn, True);
  
  // 2. Initialize Graphics
  RenderBitmap := TBitmap.Create;
  RenderBitmap.Width := WIZ_WIDTH;
  RenderBitmap.Height := WIZ_HEIGHT;
  
  BackgroundImg := TBitmapImage.Create(WizardForm);
  BackgroundImg.Parent := WizardForm;
  BackgroundImg.SetBounds(0, 0, WIZ_WIDTH, WIZ_HEIGHT);
  BackgroundImg.SendToBack;
  // BackgroundImg.OnMouseDown := @OnMouseDown; 
  
  // 3. Custom Title Bar Buttons (Manual)
  AddButton(WizardForm, 'X', False, True, WIZ_WIDTH-40, 10, 30, 30, @ActClose);
  AddButton(WizardForm, '_', False, True, WIZ_WIDTH-80, 10, 30, 30, @ActMin);
  
  // 4. Logic
  InitParticles;
  CheckIDEs;
  BuildUI;
  
  // 5. Static Render
  RenderFrame;
  // AnimTimer := TTimer.Create(WizardForm);
  // AnimTimer.Interval := 33;
  // AnimTimer.OnTimer := @OnTimer;
end;

procedure DeinitializeSetup;
begin
  RenderBitmap.Free;
end;

procedure CurPageChanged(CurPageID: Integer);
begin
  // But we hid InnerNotebook.
  // We need to parent our custom pages to the MAIN FORM or show the notebook?
  // Actually, standard Inno behavior puts them in InnerNotebook.
  // If we hide InnerNotebook, our controls disappear!
  // FIX: Unhide InnerNotebook but make it fulscreen and transparent.
  WizardForm.InnerNotebook.Visible := True;
  WizardForm.InnerNotebook.SetBounds(0, 0, WIZ_WIDTH, WIZ_HEIGHT);
  // WizardForm.InnerNotebook.Color := C_BG; // Doesn't work for Notebook
  
  // We need to move our controls explicitly? 
  // Custom Pages controls are children of Page.Surface.
  // Page.Surface is child of InnerNotebook.
  // We need to ensure backgrounds are transparent.
  // TNewMemo and TEdit are not transparent.
end;

function ShouldSkipPage(PageID: Integer): Boolean;
begin
  if (PageID = wpWelcome) or (PageID = wpSelectDir) or (PageID = wpReady) or (PageID = wpFinished) then Result := True;
end;
