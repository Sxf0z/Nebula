# Nebula Installer Backend Specification

## Installer Technology

**Choice: Inno Setup 6.x + Custom Pascal Scripts**

**Justification:**

| Criteria | Inno Setup | NSIS | Custom Rust |
|----------|------------|------|-------------|
| UI Flexibility | Good (ISSkin) | Limited | Full control |
| Speed | Excellent | Good | Excellent |
| Maintainability | Excellent | Moderate | Complex |
| Extension Support | Excellent | Excellent | Manual |
| Learning Curve | Low | Moderate | High |
| Signing Support | Built-in | Built-in | Manual |

Inno Setup wins on maintainability and extension execution. Use ISSkin or custom wizard pages for glassy UI.

---

## Core Installation Steps

### 1. Pre-Install Checks
```pascal
function InitializeSetup(): Boolean;
begin
  // Check Windows version (10+)
  // Check available disk space (50MB minimum)
  // Detect existing installation
  Result := True;
end;
```

### 2. File Installation

**Directory Structure:**
```
{userappdata}\Nebula\
├── bin\
│   └── nebula.exe          (1.2 MB)
├── lib\
│   └── std\                 (stdlib modules)
├── LICENSE
└── README.md
```

**Registry Entries (HKCU):**
```
HKCU\Software\Nebula
├── InstallPath = "C:\Users\{user}\AppData\Roaming\Nebula"
├── Version = "1.0.0"
└── InstallDate = "2026-01-17"
```

### 3. PATH Configuration

**User-level only (no admin required):**

```pascal
procedure AddToPath();
var
  Path, NewPath: string;
begin
  RegQueryStringValue(HKCU, 'Environment', 'Path', Path);
  if Pos(ExpandConstant('{app}\bin'), Path) = 0 then
  begin
    NewPath := Path + ';' + ExpandConstant('{app}\bin');
    RegWriteStringValue(HKCU, 'Environment', 'Path', NewPath);
    // Broadcast environment change
    SendBroadcastMessage(WM_SETTINGCHANGE, 0, 'Environment');
  end;
end;
```

### 4. Optional Components

| Component | Default | Action |
|-----------|---------|--------|
| Add to PATH | ON | Modify HKCU\Environment\Path |
| Desktop Shortcut | OFF | Create .lnk in {userdesktop} |
| File Association | OFF | Register .na with nebula.exe |

---

## IDE Extension Auto-Install

### Detection Strategy

```pascal
function DetectVSCode(): Boolean;
begin
  Result := FileExists(ExpandConstant('{localappdata}\Programs\Microsoft VS Code\Code.exe'))
         or FileExists('C:\Program Files\Microsoft VS Code\Code.exe')
         or RegKeyExists(HKCU, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\{EA457B21-F73E-494C-ACAB-524FDE069978}_is1');
end;

function DetectNeovim(): Boolean;
begin
  Result := FileExists(ExpandConstant('{localappdata}\nvim\bin\nvim.exe'))
         or (Exec('where', 'nvim', '', SW_HIDE, ewWaitUntilTerminated, ResultCode) and (ResultCode = 0));
end;

function DetectJetBrains(): Boolean;
var
  ToolboxPath: string;
begin
  ToolboxPath := ExpandConstant('{localappdata}\JetBrains\Toolbox');
  Result := DirExists(ToolboxPath);
end;
```

### VS Code Extension Install

**Method:** CLI invocation

```pascal
procedure InstallVSCodeExtension();
var
  ResultCode: Integer;
  CodePath: string;
begin
  // Find code executable
  if FileExists(ExpandConstant('{localappdata}\Programs\Microsoft VS Code\bin\code.cmd')) then
    CodePath := ExpandConstant('{localappdata}\Programs\Microsoft VS Code\bin\code.cmd')
  else if FileExists('C:\Program Files\Microsoft VS Code\bin\code.cmd') then
    CodePath := 'C:\Program Files\Microsoft VS Code\bin\code.cmd'
  else
    Exit; // Not found, skip silently
    
  // Install extension
  Exec(CodePath, '--install-extension nebula-lang.nebula --force', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
  // Ignore result - extension install is best-effort
end;
```

**Portable VS Code:** Check for `{userdocs}\VSCode\Code.exe` and equivalent.

### JetBrains Plugin Install

**Method:** Copy to plugins directory

```pascal
procedure InstallJetBrainsPlugin();
var
  PluginDir: string;
  IDEs: TArrayOfString;
  i: Integer;
begin
  // Check common JetBrains config locations
  IDEs := ['IntelliJIdea', 'CLion', 'Rider', 'PyCharm', 'WebStorm', 'GoLand'];
  
  for i := 0 to Length(IDEs) - 1 do
  begin
    PluginDir := ExpandConstant('{localappdata}\JetBrains\' + IDEs[i] + '*\plugins');
    if DirExists(PluginDir) then
    begin
      // Copy bundled plugin JAR
      FileCopy(ExpandConstant('{tmp}\nebula-jetbrains.jar'), 
               PluginDir + '\nebula-jetbrains.jar', False);
    end;
  end;
end;
```

**Note:** Plugin requires IDE restart. Installer does not force restart.

### Neovim Config

**Method:** Provide optional snippet, never overwrite

```pascal
procedure InstallNeovimSupport();
var
  ConfigPath, SnippetPath: string;
begin
  ConfigPath := ExpandConstant('{userappdata}\nvim\');
  SnippetPath := ConfigPath + 'nebula-setup.lua';
  
  // Only create snippet file, never modify init.lua
  if DirExists(ConfigPath) then
  begin
    SaveStringToFile(SnippetPath,
      '-- Nebula language support' + #13#10 +
      '-- Add this to your init.lua:' + #13#10 +
      '-- require("nebula-setup")' + #13#10 +
      'vim.filetype.add({ extension = { na = "nebula" } })',
      False);
  end;
end;
```

### Other IDEs

Display post-install message:
```
For Sublime Text, Atom, or other editors:
Visit https://nebula-lang.org/editors for manual setup instructions.
```

### Extension Install Rules

1. All extension installs are **opt-in** (unchecked by default if not detected)
2. Detected IDEs are **pre-checked**
3. If install fails, **continue silently**
4. Never block installer on extension failure
5. Log all extension install attempts to `{app}\install.log`

---

## Silent Install

### Supported Flags

| Flag | Effect |
|------|--------|
| `/SILENT` | No UI, show progress only |
| `/VERYSILENT` | No UI at all |
| `/NOEXTENSIONS` | Skip all IDE extension installs |
| `/NOPATH` | Do not modify PATH |
| `/DIR="path"` | Custom install directory |
| `/LOG="file"` | Write install log |

### Example
```cmd
nebula-setup.exe /VERYSILENT /NOEXTENSIONS /DIR="D:\Tools\Nebula"
```

---

## Uninstall Behavior

### Removed
- All files in `{app}\`
- Registry keys under `HKCU\Software\Nebula`
- PATH entry (restored to original)
- Desktop shortcut (if exists)
- File association (if registered)

### Preserved
- IDE extensions (user may still want them for other Nebula installs)
- User config files outside install directory

**Rationale:** IDE extensions have no side effects when Nebula is missing. Removing them risks breaking user workflows if they have multiple Nebula versions or builds from source.

---

## Upgrade Strategy

### Detection
```pascal
function IsUpgrade(): Boolean;
begin
  Result := RegKeyExists(HKCU, 'Software\Nebula');
end;
```

### Behavior
1. Read existing `InstallPath` from registry
2. Backup `{app}\config\` if exists
3. Overwrite all binaries
4. Restore backup
5. Skip extension install if version matches

### Version Comparison
```pascal
function ShouldInstallExtensions(): Boolean;
var
  InstalledVer: string;
begin
  RegQueryStringValue(HKCU, 'Software\Nebula', 'ExtensionVersion', InstalledVer);
  Result := (InstalledVer <> '{#ExtVersion}');
end;
```

---

## Security & Permissions

### Admin Rights
- **Not required** for default install
- Prompt only if user selects system-wide install (Program Files)
- User-level install to `{userappdata}` is default

### Code Signing
- Sign `nebula-setup.exe` with EV certificate
- Sign `nebula.exe` binary
- Timestamp all signatures

### Verification
```cmd
signtool verify /pa /v nebula-setup.exe
```

---

## Performance Requirements

| Metric | Target |
|--------|--------|
| Installer launch | < 300ms |
| Full install (SSD) | < 3s |
| Full install (HDD) | < 8s |
| Silent install | < 2s |
| Uninstall | < 1s |

### Optimization
- LZMA2 compression (best ratio)
- Single-threaded extract (simpler, sufficient)
- No disk-intensive operations on UI thread

---

## Packaging

### Build Script
```iss
[Setup]
AppName=Nebula
AppVersion=1.0.0
DefaultDirName={userappdata}\Nebula
OutputBaseFilename=nebula-setup
Compression=lzma2/ultra64
SolidCompression=yes
PrivilegesRequired=lowest
SignTool=signtool sign /tr http://timestamp.digicert.com /td sha256 /fd sha256 /a $f

[Files]
Source: "dist\nebula.exe"; DestDir: "{app}\bin"
Source: "dist\lib\*"; DestDir: "{app}\lib"; Flags: recursesubdirs
Source: "extensions\nebula-jetbrains.jar"; DestDir: "{tmp}"; Flags: deleteafterinstall

[Icons]
Name: "{userdesktop}\Nebula"; Filename: "{app}\bin\nebula.exe"; Tasks: desktopicon

[Tasks]
Name: "addtopath"; Description: "Add Nebula to PATH"; GroupDescription: "Configuration:"
Name: "desktopicon"; Description: "Create desktop shortcut"; GroupDescription: "Configuration:"; Flags: unchecked
Name: "vscode"; Description: "Install VS Code extension"; GroupDescription: "IDE Extensions:"; Check: DetectVSCode
Name: "neovim"; Description: "Install Neovim support"; GroupDescription: "IDE Extensions:"; Check: DetectNeovim

[Run]
Filename: "{code:GetCodePath}"; Parameters: "--install-extension nebula-lang.nebula"; Flags: nowait skipifsilent; Tasks: vscode
```

### Final Artifact
```
nebula-setup.exe    (~4 MB, signed)
```

### Version Metadata
Embedded in EXE via Inno Setup:
- ProductName: Nebula
- ProductVersion: 1.0.0.0
- CompanyName: Nebula Contributors
- LegalCopyright: MIT License
