@REM batch file to push code to github easily

git add .
git commit -m %1
git push -u origin main