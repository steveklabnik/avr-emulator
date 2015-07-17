### Installing

``` bash
npm install
```

### Building

``` bash
./node_modules/webpack/bin/webpack.js
```

### Viewing

``` bash
# Needs websocket server running
open ./index.html
```

### Testing

_This is not working currently because we're using React 0.14 and they [accidentally dropped the TestUtils package](https://github.com/facebook/react/issues/4279)._

*Options*

* Revert to React 0.13
* Find the TestUtils source and copy it into our repo

```
npm test
```
