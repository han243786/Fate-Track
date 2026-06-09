@echo off
REM BaZi Model Launcher for Windows
REM -------------------------------
REM This batch file starts the Streamlit front-end for the BaZi model.
REM Requirements: Python installed and available in your PATH.
REM If Streamlit is not installed, run the following command in a command prompt:
REM     pip install streamlit

REM Change directory to the folder containing this batch file and the Python scripts
cd /d %~dp0

REM Launch the Streamlit app (web interface). If you prefer the CLI, comment this line
python -m streamlit run app.py -- --web

REM Example CLI usage (uncomment the following line to run from the command line):
REM python app.py --pillars 甲子 丙申 壬午 庚辰

pause