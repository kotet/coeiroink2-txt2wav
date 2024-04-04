@echo off

@REM This script converts a text file dropped on it to a MP3 file using coeiroink2-txt2wav

set CONVERTER="%~dp0target\release\coeiroink2-txt2wav.exe"
set FDKAAC="fdkaac.exe"

if [%1]==[] goto :eof
:loop

echo Input file: %1
echo Output file: %1.wav

%CONVERTER% predict -i %1 -o %1.wav
%FDKAAC% -b 200704 -o %1.m4a %1.wav
del %1.wav

shift
if not [%1]==[] goto loop

pause
