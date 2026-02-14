; J Programming Language - Professional Installer
; Similar to Python, Node.js, and Visual Studio installers

#define MyAppName "J Programming Language"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "J Language Team"
#define MyAppURL "https://github.com/Llunarstack/j"
#define MyAppExeName "j.exe"

[Setup]
; Basic Information
AppId={{8F9A3B2C-1D4E-5F6A-7B8C-9D0E1F2A3B4C}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}/issues
AppUpdatesURL={#MyAppURL}/releases
AppCopyright=Copyright (C) 2024-2026 {#MyAppPublisher}

; Installation Directories
DefaultDirName={autopf}\J
DisableDirPage=no
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=no

; Output
OutputDir=..\dist\installers
OutputBaseFilename=j-lang-{#MyAppVersion}-windows-x64-setup
SetupIconFile=compiler:SetupClassicIcon.ico
UninstallDisplayIcon={app}\{#MyAppExeName}

; Compression
Compression=lzma2/ultra64
SolidCompression=yes
LZMAUseSeparateProcess=yes
LZMADictionarySize=1048576
LZMANumFastBytes=273

; Visual Style
WizardStyle=modern
WizardSizePercent=120,100

; Architecture
ArchitecturesInstallIn64BitMode=x64compatible
ArchitecturesAllowed=x64compatible

; Privileges
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=commandline dialog

; License
LicenseFile=..\..\LICENSE

; Misc
AllowNoIcons=yes
DisableWelcomePage=no
DisableReadyPage=no
ShowLanguageDialog=auto
SetupLogging=yes
VersionInfoVersion={#MyAppVersion}
VersionInfoCompany={#MyAppPublisher}
VersionInfoDescription={#MyAppName} Setup
VersionInfoCopyright=Copyright (C) 2024-2026 {#MyAppPublisher}
VersionInfoProductName={#MyAppName}
VersionInfoProductVersion={#MyAppVersion}

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Types]
Name: "full"; Description: "Full Installation (Recommended)"
Name: "minimal"; Description: "Minimal Installation"
Name: "custom"; Description: "Custom Installation"; Flags: iscustom

[Components]
Name: "core"; Description: "J Language Core"; Types: full minimal custom; Flags: fixed
Name: "examples"; Description: "Example Programs"; Types: full
Name: "docs"; Description: "Documentation"; Types: full
Name: "stdlib"; Description: "Standard Library"; Types: full minimal

[Tasks]
Name: "addtopath"; Description: "Add J to the system PATH environment variable"; GroupDescription: "Environment:"; Flags: checkedonce
Name: "addallusers"; Description: "Add to PATH for all users (requires admin)"; GroupDescription: "Environment:"; Flags: unchecked; Check: IsAdminInstallMode
Name: "fileassoc"; Description: "Associate .j files with J"; GroupDescription: "File Associations:"; Flags: checkedonce
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional Shortcuts:"; Flags: unchecked
Name: "quicklaunch"; Description: "Create a Quick Launch shortcut"; GroupDescription: "Additional Shortcuts:"; Flags: unchecked; OnlyBelowVersion: 6.1; Check: not IsAdminInstallMode

[Files]
; Core executable
Source: "..\dist\j-windows-x86_64.exe"; DestDir: "{app}"; DestName: "j.exe"; Flags: ignoreversion; Components: core

; Icon file (real ICO format)
Source: "..\J_lang.ico"; DestDir: "{app}"; Flags: ignoreversion; Components: core

; Standard Library
Source: "..\lib\*"; DestDir: "{app}\lib"; Flags: ignoreversion recursesubdirs createallsubdirs; Components: stdlib

; Examples
Source: "..\examples\*.j"; DestDir: "{app}\examples"; Flags: ignoreversion; Components: examples
Source: "..\examples\basic\*.j"; DestDir: "{app}\examples\basic"; Flags: ignoreversion; Components: examples

; Documentation
Source: "..\..\README.md"; DestDir: "{app}"; DestName: "README.txt"; Flags: ignoreversion isreadme; Components: docs
Source: "..\..\LICENSE"; DestDir: "{app}"; DestName: "LICENSE.txt"; Flags: ignoreversion; Components: docs
Source: "..\..\CONTRIBUTING.md"; DestDir: "{app}"; DestName: "CONTRIBUTING.txt"; Flags: ignoreversion; Components: docs
Source: "..\..\CHANGELOG.md"; DestDir: "{app}"; DestName: "CHANGELOG.txt"; Flags: ignoreversion; Components: docs

[Icons]
; Start Menu
Name: "{group}\J REPL"; Filename: "{app}\{#MyAppExeName}"; Parameters: "repl"; Comment: "Start J Interactive REPL"; IconFilename: "{app}\J_lang.ico"
Name: "{group}\J Documentation"; Filename: "{app}\README.txt"; Comment: "View J Documentation"
Name: "{group}\Example Programs"; Filename: "{app}\examples"; Comment: "Browse Example Programs"
Name: "{group}\Standard Library"; Filename: "{app}\lib"; Comment: "Browse Standard Library"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"; Comment: "Uninstall J Programming Language"

; Desktop
Name: "{autodesktop}\J REPL"; Filename: "{app}\{#MyAppExeName}"; Parameters: "repl"; Comment: "Start J Interactive REPL"; IconFilename: "{app}\J_lang.ico"; Tasks: desktopicon

; Quick Launch
Name: "{userappdata}\Microsoft\Internet Explorer\Quick Launch\J REPL"; Filename: "{app}\{#MyAppExeName}"; Parameters: "repl"; IconFilename: "{app}\J_lang.ico"; Tasks: quicklaunch

[Registry]
; File Association
Root: HKA; Subkey: "Software\Classes\.j"; ValueType: string; ValueName: ""; ValueData: "JSourceFile"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\.j"; ValueType: string; ValueName: "Content Type"; ValueData: "text/plain"; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\.j"; ValueType: string; ValueName: "PerceivedType"; ValueData: "text"; Tasks: fileassoc

Root: HKA; Subkey: "Software\Classes\JSourceFile"; ValueType: string; ValueName: ""; ValueData: "J Source File"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\JSourceFile"; ValueType: string; ValueName: "FriendlyTypeName"; ValueData: "J Source File"; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\JSourceFile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\J_lang.ico,0"; Tasks: fileassoc

Root: HKA; Subkey: "Software\Classes\JSourceFile\shell"; ValueType: string; ValueName: ""; ValueData: "open"; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\JSourceFile\shell\open"; ValueType: string; ValueName: ""; ValueData: "Run with J"; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\JSourceFile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" run ""%1"""; Tasks: fileassoc

Root: HKA; Subkey: "Software\Classes\JSourceFile\shell\edit"; ValueType: string; ValueName: ""; ValueData: "Edit"; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\JSourceFile\shell\edit\command"; ValueType: string; ValueName: ""; ValueData: "notepad.exe ""%1"""; Tasks: fileassoc

; Application Registration
Root: HKCU; Subkey: "Software\{#MyAppPublisher}\{#MyAppName}"; ValueType: string; ValueName: "InstallPath"; ValueData: "{app}"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\{#MyAppPublisher}\{#MyAppName}"; ValueType: string; ValueName: "Version"; ValueData: "{#MyAppVersion}"

[Run]
; Post-installation actions
Filename: "{app}\{#MyAppExeName}"; Parameters: "--version"; StatusMsg: "Verifying installation..."; Flags: runhidden waituntilterminated
Filename: "{app}\README.txt"; Description: "View the README file"; Flags: postinstall shellexec skipifsilent unchecked
Filename: "{app}\{#MyAppExeName}"; Parameters: "repl"; Description: "Launch J REPL"; Flags: postinstall nowait skipifsilent unchecked

[UninstallRun]
; Cleanup before uninstall
Filename: "{cmd}"; Parameters: "/C ""taskkill /F /IM j.exe /T"""; Flags: runhidden; RunOnceId: "KillJProcess"

[UninstallDelete]
Type: filesandordirs; Name: "{app}"

[Code]
var
  ProgressPage: TOutputProgressWizardPage;
  
// Custom welcome message
procedure InitializeWizard();
begin
  // Create custom progress page
  ProgressPage := CreateOutputProgressPage('Installing J Language', 'Please wait while Setup installs J Programming Language on your computer.');
end;

// Add to PATH
procedure AddToPath(PathToAdd: string; AllUsers: Boolean);
var
  RegKey: Integer;
  RegPath: string;
  OldPath: string;
  NewPath: string;
begin
  if AllUsers then
  begin
    RegKey := HKEY_LOCAL_MACHINE;
    RegPath := 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';
  end
  else
  begin
    RegKey := HKEY_CURRENT_USER;
    RegPath := 'Environment';
  end;

  if RegQueryStringValue(RegKey, RegPath, 'Path', OldPath) then
  begin
    // Check if already in PATH
    if Pos(Uppercase(PathToAdd), Uppercase(OldPath)) = 0 then
    begin
      // Add to PATH
      if OldPath <> '' then
        NewPath := OldPath + ';' + PathToAdd
      else
        NewPath := PathToAdd;
      
      if RegWriteExpandStringValue(RegKey, RegPath, 'Path', NewPath) then
      begin
        Log('Added to PATH: ' + PathToAdd);
      end;
    end
    else
    begin
      Log('Already in PATH: ' + PathToAdd);
    end;
  end
  else
  begin
    // Create new PATH
    if RegWriteExpandStringValue(RegKey, RegPath, 'Path', PathToAdd) then
    begin
      Log('Created PATH with: ' + PathToAdd);
    end;
  end;
end;

// Remove from PATH
procedure RemoveFromPath(PathToRemove: string; AllUsers: Boolean);
var
  RegKey: Integer;
  RegPath: string;
  OldPath: string;
  NewPath: string;
  P: Integer;
begin
  if AllUsers then
  begin
    RegKey := HKEY_LOCAL_MACHINE;
    RegPath := 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';
  end
  else
  begin
    RegKey := HKEY_CURRENT_USER;
    RegPath := 'Environment';
  end;

  if RegQueryStringValue(RegKey, RegPath, 'Path', OldPath) then
  begin
    NewPath := OldPath;
    
    // Remove ;PathToRemove
    StringChangeEx(NewPath, ';' + PathToRemove, '', True);
    
    // Remove PathToRemove;
    StringChangeEx(NewPath, PathToRemove + ';', '', True);
    
    // Remove PathToRemove (if it's the only entry)
    if NewPath = PathToRemove then
      NewPath := '';
    
    if NewPath <> OldPath then
    begin
      if RegWriteExpandStringValue(RegKey, RegPath, 'Path', NewPath) then
      begin
        Log('Removed from PATH: ' + PathToRemove);
      end;
    end;
  end;
end;

// Post-install
procedure CurStepChanged(CurStep: TSetupStep);
var
  AppPath: string;
  AllUsers: Boolean;
  ResultCode: Integer;
begin
  if CurStep = ssPostInstall then
  begin
    if WizardIsTaskSelected('addtopath') then
    begin
      AppPath := ExpandConstant('{app}');
      AllUsers := WizardIsTaskSelected('addallusers') and IsAdminInstallMode;
      
      ProgressPage.SetText('Configuring environment variables...', '');
      ProgressPage.Show;
      try
        AddToPath(AppPath, AllUsers);
      finally
        ProgressPage.Hide;
      end;
    end;
    
    // Refresh icon cache and file associations
    if WizardIsTaskSelected('fileassoc') then
    begin
      ProgressPage.SetText('Refreshing file associations...', '');
      ProgressPage.Show;
      try
        // Notify Windows of file association changes
        Exec('cmd.exe', '/c assoc .j=JSourceFile', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
        Exec('cmd.exe', '/c ftype JSourceFile="' + ExpandConstant('{app}\{#MyAppExeName}') + '" run "%1"', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
        
        // Refresh icon cache
        Exec('ie4uinit.exe', '-show', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
      finally
        ProgressPage.Hide;
      end;
    end;
  end;
end;

// Uninstall
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  AppPath: string;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    AppPath := ExpandConstant('{app}');
    
    // Try to remove from both user and system PATH
    RemoveFromPath(AppPath, False);
    if IsAdminInstallMode then
      RemoveFromPath(AppPath, True);
  end;
end;

// Custom finish message
function UpdateReadyMemo(Space, NewLine, MemoUserInfoInfo, MemoDirInfo, MemoTypeInfo, MemoComponentsInfo, MemoGroupInfo, MemoTasksInfo: String): String;
var
  S: String;
begin
  S := '';
  
  S := S + 'Installation Summary:' + NewLine + NewLine;
  S := S + MemoDirInfo + NewLine + NewLine;
  S := S + MemoGroupInfo + NewLine + NewLine;
  
  if MemoComponentsInfo <> '' then
    S := S + MemoComponentsInfo + NewLine + NewLine;
  
  if MemoTasksInfo <> '' then
    S := S + MemoTasksInfo + NewLine + NewLine;
  
  S := S + 'After installation:' + NewLine;
  S := S + '  - Run "j --version" to verify installation' + NewLine;
  S := S + '  - Run "j repl" to start the interactive REPL' + NewLine;
  S := S + '  - Run "j run <file.j>" to execute a J program' + NewLine;
  
  Result := S;
end;
