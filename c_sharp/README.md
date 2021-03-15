C Sharp
===

## Run

```powershell
cd HelloWorld

Add-Type -TypeDefinition ([String](Get-Content -Path ".\HelloWorld.cs" -Raw)) -Language CSharp

[Sample]::Main()
```

## Build EXE file

```powershell
cd HelloWorld

Add-Type -path ".\HelloWorld.cs" `
    -OutputAssembly HelloWorld.exe `
    -OutputType ConsoleApplication
```
