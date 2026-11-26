; Yapfast's Windows installer. scripts/package-windows.py compiles it with
; Inno Setup 7 under Wine, from the same files as the portable ZIP.
;
; It installs for the person running it, without administrator rights, and can
; start Yapfast in the tray when they sign in. That is the Run key value
; Settings > Advanced > "Start when you sign in" writes (src/autostart.rs), so
; the installer and the setting turn the same thing on and off.

#ifndef AppVersion
  #error Compile through scripts/package-windows.py, which defines AppVersion, VersionNumber and SourceDir
#endif
; The full preview defines Edition and FileSuffix; the Discord release neither.
; Both share the AppId, so installing one replaces the other.
#ifdef Edition
  #define DisplayName "Discy (" + Edition + ")"
#else
  #define DisplayName "Discy"
  #define FileSuffix ""
#endif

[Setup]
; Windows knows an installed Discy by this; never change it.
AppId={{19FF8D53-8D7E-4E66-91BB-E2F5BE307E9A}
AppName=Discy
AppVersion={#AppVersion}
AppVerName={#DisplayName} {#AppVersion}
AppPublisher=Discy
AppPublisherURL=https://github.com/x2super/Discy
AppSupportURL=https://github.com/x2super/Discy/issues
AppUpdatesURL=https://github.com/x2super/Discy/releases
VersionInfoVersion={#VersionNumber}
; Per user: %LOCALAPPDATA%\Programs\Discy, with no UAC prompt.
PrivilegesRequired=lowest
DefaultDirName={autopf}\Discy
DisableProgramGroupPage=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
OutputBaseFilename=Discy-{#AppVersion}{#FileSuffix}-windows-x64-setup
SetupIconFile=yapfast.ico
UninstallDisplayIcon={app}\discy.exe
UninstallDisplayName={#DisplayName}
WizardStyle=modern
Compression=lzma2/max
SolidCompression=yes
; Discy keeps running in the tray with its window closed, so an update or
; uninstall closes it rather than asking.
CloseApplications=force
RestartApplications=no
#ifdef LicenseFile
LicenseFile={#LicenseFile}
#endif

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[CustomMessages]
english.StartOnSignIn=Start Discy automatically when Windows starts
english.StartOnSignInGroup=Startup:

[Tasks]
Name: "startonsignin"; Description: "{cm:StartOnSignIn}"; GroupDescription: "{cm:StartOnSignInGroup}"
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
; README.txt explains the ZIP; the installer needs no instructions.
Source: "{#SourceDir}\*"; Excludes: "README.txt"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{autoprograms}\Discy"; Filename: "{app}\discy.exe"
Name: "{autodesktop}\Discy"; Filename: "{app}\discy.exe"; Tasks: desktopicon

[Registry]
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "Discy"; ValueData: """{app}\discy.exe"" --background"; Tasks: startonsignin
; Chosen here, it starts even if Task Manager had turned it off before.
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run"; ValueType: none; ValueName: "Discy"; Flags: deletevalue dontcreatekey; Tasks: startonsignin
; Discy's own links open here; claiming Discord's stays the person's choice.
Root: HKCU; Subkey: "Software\Classes\discy"; ValueType: string; ValueName: ""; ValueData: "URL:Discy Protocol"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\discy"; ValueType: string; ValueName: "URL Protocol"; ValueData: ""; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\discy\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: """{app}\discy.exe"",0"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\discy\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\discy.exe"" --open-link ""%1"""; Flags: uninsdeletekey
; Settings keeps the app discord:// links opened before claiming them here;
; uninstalling gives them back (below) and drops what was kept.
Root: HKCU; Subkey: "Software\Discy"; Flags: uninsdeletekeyifempty dontcreatekey
Root: HKCU; Subkey: "Software\Discy\DiscordProtocol"; Flags: uninsdeletekey dontcreatekey
; Each start registers Discy's name and icon for its notifications
; (src/notify.rs); uninstalling forgets them.
Root: HKCU; Subkey: "Software\Classes\AppUserModelId\io.github.discy.Discy"; Flags: uninsdeletekey dontcreatekey
; Uninstalling stops starting at sign-in, however it was turned on.
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: none; ValueName: "Discy"; Flags: uninsdeletevalue dontcreatekey
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run"; ValueType: none; ValueName: "Discy"; Flags: uninsdeletevalue dontcreatekey

[Run]
Filename: "{app}\discy.exe"; Description: "{cm:LaunchProgram,Discy}"; Flags: nowait postinstall skipifsilent

[UninstallRun]
; Before the files go: discord:// links open in the app that had them again,
; if Settings claimed them for this Discy.
Filename: "{app}\discy.exe"; Parameters: "--release-discord-links"; Flags: runhidden waituntilterminated; RunOnceId: "ReleaseDiscordLinks"
