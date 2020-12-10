const http = require("http");
const fs = require('fs');

const server = http.createServer(function (req, res) {
  const url = (req.url.endsWith("/") ? req.url + "index.html" : req.url);
  if (fs.existsSync(url)) {
    fs.readFile(url, (err, data) => {
      if (!err) {
        res.writeHead(200, {"Content-Type": "text/html"});
        res.end(data);
      }
    });
  } else if (req.url === "/aaa") {
    res.writeHead(200, {"Content-Type": "text/html"});
    res.end('{"data":"aaa"}');
  } else {
    res.writeHead(302, {
      'Location': 'http://localhost:5000'
    });
    res.end();
  }
});

const port = process.env.PORT || 5000;
server.listen(port, function() {
  console.log("To view your app, open this link in your browser: http://localhost:" + port);
});
