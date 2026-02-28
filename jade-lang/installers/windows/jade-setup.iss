; Inno Setup script for Jade Language
; Build: iscc jade-setup.iss [/DBinaryRelPath=..\..\target\release] [/DOutputArch=x86_64]
; Default: x86_64, binary from ..\..\target\release

#define MyAppName "Jade"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Jade Language"
#define MyAppURL "https://github.com/Llunarstack/j"
#define MyAppExeName "jade.exe"
; Start Menu folder: "Jade 1.0.0". Shortcut: "Jade 1.0.0 (64-bit)" etc.
#define MyAppGroup "Jade 1.0.0"
#ifndef ShortcutSuffix
#define ShortcutSuffix "(64-bit)"
#endif
#ifndef BinaryRelPath
#define BinaryRelPath "..\..\target\release"
#endif
#ifndef OutputArch
#define OutputArch "x86_64"
#endif

[Setup]
AppId={{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\Jade 1.0.0
DefaultGroupName={#MyAppGroup}
DisableProgramGroupPage=yes
SetupIconFile=.\icon\jade.ico
UninstallDisplayIcon={app}\{#MyAppExeName}
OutputDir=..\..\..\dist\installers\windows
OutputBaseFilename=jade-{#MyAppVersion}-windows-{#OutputArch}-setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
ArchitecturesAllowed=x64 x86 arm64
ArchitecturesInstallIn64BitMode=x64 arm64

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "addtopath"; Description: "Add Jade to PATH (run ""jade"" from any terminal)"; GroupDescription: "Options:"; Flags: checkedonce
Name: "installext"; Description: "Install Jade extension for VS Code and Cursor (syntax, run from buffer, autosave)"; GroupDescription: "Options:"; Flags: checkedonce
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "associatejade"; Description: "Associate .jdl files with Jade (run/interpret)"; GroupDescription: "File associations:"; Flags: checkedonce

[Files]
; Use JadeExePath (absolute) when passed by build-exe.ps1 so the correct exe is always packaged.
#ifdef JadeExePath
Source: "{#JadeExePath}"; DestDir: "{app}"; Flags: ignoreversion
#else
Source: "{#BinaryRelPath}\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
#endif
; Icon for .jdl association (exe also has icon embedded)
Source: ".\icon\jade.ico"; DestDir: "{app}"; Flags: ignoreversion
; Doc, Lib, Scripts, LICENSE, NEWS (Python-style install layout)
Source: ".\install-extras\Doc\*"; DestDir: "{app}\Doc"; Flags: ignoreversion recursesubdirs
Source: ".\install-extras\Lib\*"; DestDir: "{app}\Lib"; Flags: ignoreversion recursesubdirs
Source: ".\install-extras\Scripts\*"; DestDir: "{app}\Scripts"; Flags: ignoreversion recursesubdirs
Source: ".\install-extras\LICENSE.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: ".\install-extras\NEWS.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: ".\install-extras\BUILD.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: ".\install-extras\install-ide-extension.ps1"; DestDir: "{app}"; Flags: ignoreversion
; VS Code / Cursor extension (syntax, run from buffer, debounced autosave) - path from build-exe.ps1
#ifdef IdeExtPath
Source: "{#IdeExtPath}\*"; DestDir: "{app}\ide\vscode"; Flags: ignoreversion recursesubdirs
#else
Source: "..\ide\vscode-snippet\*"; DestDir: "{app}\ide\vscode"; Flags: ignoreversion recursesubdirs
#endif

[Icons]
Name: "{group}\Jade"; Filename: "{app}\{#MyAppExeName}"; Parameters: "repl"
Name: "{group}\Jade IDLE"; Filename: "{app}\{#MyAppExeName}"; Parameters: "idle"
Name: "{group}\Uninstall Jade {#MyAppVersion}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\Jade"; Filename: "{app}\{#MyAppExeName}"; Parameters: "repl"; Tasks: desktopicon

[Registry]
; Associate .jdl with Jade: run with jade.exe and use Jade logo (no IDE extension needed)
Root: HKCR; Subkey: ".jdl"; ValueType: string; ValueName: ""; ValueData: "Jade.SourceFile"; Flags: uninsdeletevalue; Tasks: associatejade
Root: HKCR; Subkey: ".jdl\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\jade.ico"; Flags: uninsdeletevalue; Tasks: associatejade
Root: HKCR; Subkey: "Jade.SourceFile"; ValueType: string; ValueName: ""; ValueData: "Jade source file"; Flags: uninsdeletekey; Tasks: associatejade
Root: HKCR; Subkey: "Jade.SourceFile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\jade.ico"; Flags: uninsdeletevalue; Tasks: associatejade
Root: HKCR; Subkey: "Jade.SourceFile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""; Flags: uninsdeletevalue; Tasks: associatejade

[Code]
const
  HWND_TOPMOST = -1;
  HWND_NOTOPMOST = -2;
  SWP_NOMOVE = $0002;
  SWP_NOSIZE = $0001;

function SetForegroundWindow(hWnd: HWND): BOOL; external 'SetForegroundWindow@user32.dll stdcall';
function GetForegroundWindow: HWND; external 'GetForegroundWindow@user32.dll stdcall';
function GetWindowThreadProcessId(hWnd: HWND; var lpdwProcessId: LongWord): LongWord; external 'GetWindowThreadProcessId@user32.dll stdcall';
function GetCurrentThreadId: LongWord; external 'GetCurrentThreadId@kernel32.dll stdcall';
function AttachThreadInput(idAttach, idAttachTo: LongWord; fAttach: BOOL): BOOL; external 'AttachThreadInput@user32.dll stdcall';
function SetWindowPos(hWnd, hWndInsertAfter: HWND; X, Y, cx, cy: Integer; uFlags: UINT): BOOL; external 'SetWindowPos@user32.dll stdcall';

procedure BringWizardToFront;
var
  ForegroundWnd: HWND;
  ForegroundThread, OurThread: LongWord;
  DummyPid: LongWord;
begin
  WizardForm.BringToFront;
  WizardForm.Show;
  { Force window on top: briefly set TOPMOST so it appears above everything }
  SetWindowPos(WizardForm.Handle, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE or SWP_NOSIZE);
  SetWindowPos(WizardForm.Handle, HWND_NOTOPMOST, 0, 0, 0, 0, SWP_NOMOVE or SWP_NOSIZE);
  { AttachThreadInput workaround so SetForegroundWindow is allowed by Windows }
  ForegroundWnd := GetForegroundWindow;
  if ForegroundWnd <> 0 then
  begin
    ForegroundThread := GetWindowThreadProcessId(ForegroundWnd, DummyPid);
    OurThread := GetCurrentThreadId;
    if AttachThreadInput(OurThread, ForegroundThread, True) then
    begin
      SetForegroundWindow(WizardForm.Handle);
      AttachThreadInput(OurThread, ForegroundThread, False);
    end
    else
      SetForegroundWindow(WizardForm.Handle);
  end
  else
    SetForegroundWindow(WizardForm.Handle);
end;

procedure CurPageChanged(CurPageID: Integer);
begin
  if CurPageID = wpWelcome then
    BringWizardToFront;
end;

procedure InitializeWizard;
begin
  BringWizardToFront;
end;

procedure EnvAddPath(Path: string);
var
  Paths: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', Paths) then
    Paths := '';
  if Pos(';' + Path + ';', ';' + Paths + ';') = 0 then
    RegWriteStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', Paths + ';' + Path);
end;

procedure EnvRemovePath(Path: string);
var
  Paths: string;
  P: Integer;
begin
  if RegQueryStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', Paths) then
  begin
    { Search in ';' + Paths + ';' so we match boundaries; P is position in that string }
    P := Pos(';' + Path + ';', ';' + Paths + ';');
    if P > 0 then
      { In Paths, the leading ';' is at index P-1 (extended string has ';' at 1, Paths at 2..) }
      Delete(Paths, P - 1, Length(Path) + 1)
    else if Pos(Path + ';', Paths) = 1 then
      Delete(Paths, 1, Length(Path) + 1)
    else if Pos(';' + Path, Paths) > 0 then
      Delete(Paths, Pos(';' + Path, Paths), Length(Path) + 1);
    RegWriteStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', Paths);
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
  EnvPath: string;
  AppPath: string;
  ResultCode: Integer;
begin
  if CurStep = ssPostInstall then
  begin
    if WizardIsTaskSelected('addtopath') then
    begin
      EnvPath := ExpandConstant('{app}');
      if Pos(';' + EnvPath + ';', ';' + GetEnv('Path') + ';') = 0 then
        EnvAddPath(EnvPath);
    end;
    if WizardIsTaskSelected('installext') then
    begin
      AppPath := ExpandConstant('{app}');
      Exec(ExpandConstant('{sys}\WindowsPowerShell\v1.0\powershell.exe'),
        '-NoProfile -ExecutionPolicy Bypass -File "' + AppPath + '\install-ide-extension.ps1" -JadeAppPath "' + AppPath + '"',
        '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
    end;
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
    EnvRemovePath(ExpandConstant('{app}'));
end;
