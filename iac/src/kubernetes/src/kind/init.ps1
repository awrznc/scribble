$DistroName = 'kind'

# 作業用ディレクトリの作成
$WorkDir = $PSScriptRoot + '\' + '.tmp'
mkdir $WorkDir
cd $WorkDir

# イメージの取得
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri https://aka.ms/wsl-debian-gnulinux -OutFile debian.zip -UseBasicParsing
Expand-Archive -Path debian.zip -DestinationPath debian\
Rename-Item debian\DistroLauncher-Appx_1.12.2.0_x64.appx DistroLauncher.zip
Expand-Archive -Path debian\DistroLauncher.zip -DestinationPath launcher\
cd $PSScriptRoot

# import
mkdir $DistroName
wsl --import $DistroName $DistroName\ $WorkDir\launcher\install.tar.gz

# check
wsl -l -v
wsl -d $DistroName cat /etc/debian_version
# => 11.3

wsl -d $DistroName ./init.sh
