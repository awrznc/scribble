' Version check
MsgBox(ScriptEngineMajorVersion & "." & ScriptEngineMinorVersion)

' Variable
Dim string
string = "Date: " & Date & vbCr & "Time: " & Time
MsgBox( string )

' Title
MsgBox "text", , "title"

' ARGV
Dim inputValue
inputValue = InputBox( "Please input string.", "input" )
MsgBox( inputValue )

' Function
Function SampleFunction(value)
  SampleFunction = value
End Function
Dim sampleString
sampleString = SampleFunction("sample")
MsgBox( sampleString )
