# https://ux.getuploader.com/touhougc/download/4
$DownloadUri = 'https://downloadx.getuploader.com/g/67bdd41e-a0c4-4d30-ae37-3fdfa010e467/touhougc/4/th_dnh_ph3.zip'
$DownloadFile = 'th_dnh_ph3.zip'

# 作業用ディレクトリの作成
$WorkDir = $PSScriptRoot + '\' + '.tmp'
mkdir $WorkDir
cd $WorkDir

# 取得
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri $DownloadUri -OutFile $DownloadFile -UseBasicParsing
Expand-Archive -Path $DownloadFile -DestinationPath $WorkDir\
