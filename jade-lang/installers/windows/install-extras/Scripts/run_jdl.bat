@echo off
REM Run a .jdl file with Jade (use if jade is on PATH)
if "%~1"=="" (
  echo Usage: run_jdl.bat script.jdl
  exit /b 1
)
jade "%~1"
pause
