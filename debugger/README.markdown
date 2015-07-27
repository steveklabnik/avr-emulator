# Deploying

### Build bundle.js

```
git co master
cd debugger
./node_modules/webpack/bin/webpack.js --config webpack-production.config.js
```

### Update `gh-pages` branch

```
cd ../
git co gh-pages
git co master -- debugger/index.html
mv debugger/index.html ./
sed 's/localhost:8000/72.2.112.229:8000/g' debugger/dist/bundle.js > bundle.js
```

### Push change to `origin/gh-pages`

```
git add .
git commit -m 'updated github pages build'
git push
```

### Move back to `master`

```
git co master
```
