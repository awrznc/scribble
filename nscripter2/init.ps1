$DownloadUri = 'https://www.nscripter.com/nscr2.zip'
$DownloadFile = 'nscr2.zip'

# 作業用ディレクトリの作成
$WorkDir = $PSScriptRoot + '\' + '.tmp'
mkdir $WorkDir
cd $WorkDir

# 取得
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri $DownloadUri -OutFile $DownloadFile -UseBasicParsing
Expand-Archive -Path $DownloadFile -DestinationPath $WorkDir\
