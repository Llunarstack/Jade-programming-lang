; J Language - Inno Setup Installer Script
; Creates a professional Windows installer with GUI

#define MyAppName "J Programming Language"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "J Language Team"
#define MyAppURL "https://github.com/j-lang/j"
#define MyAppExeName "j.exe"

[Setup]
; Basic information
AppId={{8F9A3B2C-1D4E-5F6A-7B8C-9D0E1F2A3B4C}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\J
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
OutputDir=..\dist\installers
OutputBaseFilename=j-lang-{#MyAppVersion}-windows-setup
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64compatible
ArchitecturesAllowed=x86 x64compatible arm64

; Privileges
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "addtopath"; Description: "Add J to system PATH (recommended)"; GroupDescription: "System Integration:"; Flags: checkedonce
Name: "fileassoc"; Description: "Associate .j files with J"; GroupDescription: "System Integration:"; Flags: checkedonce

[Files]
; Main executable
Source: "..\dist\j-windows-x86_64.exe"; DestDir: "{app}"; DestName: "j.exe"; Flags: ignoreversion; Check: Is64BitInstallMode
Source: "..\target\release\j.exe"; DestDir: "{app}"; Flags: ignoreversion; Check: not Is64BitInstallMode

; Examples
Source: "..\examples\*"; DestDir: "{app}\examples"; Flags: ignoreversion recursesubdirs createallsubdirs

; Documentation
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion isreadme
Source: "..\..\README.md"; DestDir: "{app}"; Flags: ignoreversion; Check: not FileExists(ExpandConstant('{app}\README.md'))

[Icons]
Name: "{group}\J REPL"; Filename: "{app}\{#MyAppExeName}"; Parameters: "repl"; IconFilename: "{app}\J_lang_logo.ico"
Name: "{group}\J Documentation"; Filename: "{app}\README.md"
Name: "{group}\Examples"; Filename: "{app}\examples"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\J REPL"; Filename: "{app}\{#MyAppExeName}"; Parameters: "repl"; IconFilename: "{app}\J_lang_logo.ico"; Tasks: desktopicon

[Registry]
; File association
Root: HKA; Subkey: "Software\Classes\.j"; ValueType: string; ValueName: ""; ValueData: "JSourceFile"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\JSourceFile"; ValueType: string; ValueName: ""; ValueData: "J Source File"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\JSourceFile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\J_lang_logo.ico"; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\JSourceFile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" run ""%1"""; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\JSourceFile\shell\edit"; ValueType: string; ValueName: ""; ValueData: "Edit with Notepad"; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\JSourceFile\shell\edit\command"; ValueType: string; ValueName: ""; ValueData: "notepad.exe ""%1"""; Tasks: fileassoc

[Code]
procedure CurStepChanged(CurStep: TSetupStep);
var
  AppPath, OldPath: string;
begin
  if (CurStep = ssPostInstall) and WizardIsTaskSelected('addtopath') then
  begin
    AppPath := ExpandConstant('{app}');
    
    // Add to user PATH
    if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OldPath) then
    begin
      if Pos(AppPath, OldPath) = 0 then
      begin
        if OldPath <> '' then
          OldPath := OldPath + ';';
        RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OldPath + AppPath);
      end;
    end
    else
    begin
      RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', AppPath);
    end;
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  AppPath, OldPath, NewPath: string;
  P: Integer;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    AppPath := ExpandConstant('{app}');
    
    if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OldPath) then
    begin
      P := Pos(';' + AppPath, OldPath);
      if P > 0 then
      begin
        NewPath := Copy(OldPath, 1, P - 1) + Copy(OldPath, P + Length(AppPath) + 1, Length(OldPath));
        RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', NewPath);
      end
      else
      begin
        P := Pos(AppPath + ';', OldPath);
        if P > 0 then
        begin
          NewPath := Copy(OldPath, 1, P - 1) + Copy(OldPath, P + Length(AppPath) + 1, Length(OldPath));
          RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', NewPath);
        end
        else if OldPath = AppPath then
        begin
          RegDeleteValue(HKEY_CURRENT_USER, 'Environment', 'Path');
        end;
      end;
    end;
  end;
end;

[Run]
Filename: "{app}\{#MyAppExeName}"; Parameters: "--version"; Description: "Verify installation"; Flags: postinstall skipifsilent nowait runhidden
Filename: "{app}\README.md"; Description: "View documentation"; Flags: postinstall shellexec skipifsilent unchecked

[UninstallDelete]
Type: filesandordirs; Name: "{app}"
