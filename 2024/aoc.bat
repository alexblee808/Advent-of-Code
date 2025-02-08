@ECHO OFF

:: Set Advent of Code variables
@REM You can find SESSION by using Chrome tools:
@REM 1) Go to https://adventofcode.com/2022/day/1/input
@REM 2) right-click -> inspect -> click the "Application" tab.
@REM 3) Refresh
@REM 5) Click https://adventofcode.com under "Cookies"
@REM 6) Grab the value for session. Fill it in.
set SESSION=53616c7465645f5f5984fbd716e839bf76b9a347720bfc5014dfde0ac4c6a5c36f07f868842930f3690af029c5dc6cc3da35920c35ea8fae854cf6264e43ff92

set year=%1
set day=%2

set padded=0%day%
set day_dir=day-%padded:~-2%

if not exist %day_dir%\ (
  mkdir %day_dir%
)
curl -b session=%SESSION% https://adventofcode.com/%year%/day/%day%/input -o %day_dir%\input1.txt
copy %day_dir%\input1.txt %day_dir%\input2.txt

@REM The ownership key is obtained after signing into the Advent of Code website, selecting **Settings**, and then hovering over the _(hover to reveal)_ box.
@REM There is no practical use for the key at this time.
set OWNERKEY="ownerproof-4803461-1734133438-b8114cb26e9a"