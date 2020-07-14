## 用語

https://docs.microsoft.com/ja-jp/cpp/build/reference/file-types-created-for-visual-cpp-projects?view=vs-2019

## Tutorial

* https://docs.microsoft.com/ja-jp/cpp/?view=vs-2019
* https://docs.microsoft.com/ja-jp/cpp/windows/overview-of-windows-programming-in-cpp?view=vs-2019
* https://docs.microsoft.com/ja-jp/cpp/build/projects-and-build-systems-cpp?view=vs-2019

## Installer

https://aka.ms/vs/16/release/vs_buildtools.exe


## Build

```powershell
# search instance id
> ${id} = (Invoke-Expression "& 'C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe' -latest -format json" | ConvertFrom-Json).instanceId

# login instance
> powershell.exe -noe -c "&{Import-Module C:\'Program Files (x86)'\'Microsoft Visual Studio'\2019\BuildTools\Common7\Tools\Microsoft.VisualStudio.DevShell.dll; Enter-VsDevShell ${id}}"
```

### cl

```powershell
# build
> cl /EHsc .\source.cpp /out:sample.exe

# run
> .\sample.exe
# => output
```

### MSBuild

```powershell
# build
> msbuild .\sample.vcxproj

# run
> .\Debug\sample.exe
# => output
```
