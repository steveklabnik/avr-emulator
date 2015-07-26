### Deploying

```
git co master
./node_modules/webpack/bin/webpack.js --config webpack-production.config.js
```

```
git co gh-pages
git co master -- debugger/index.html
mv debugger/index.html ./
sed 's/localhost:8000/72.2.112.220:8000/g' debugger/dist/bundle.js > bundle.js
```

```
git push
```

```
git co master
```

