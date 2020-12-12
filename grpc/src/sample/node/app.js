const http = require("http");
const fs = require('fs');

const server = http.createServer(function (request, response) {
  const url = "./" + (request.url.endsWith("/") ? request.url + "index.html" : request.url);
  let name = '';
  if (fs.existsSync(url)) {
    fs.readFile(url, (err, data) => {
      if (!err) {
        response.writeHead(200, {"Content-Type": "text/html"});
        response.end(data);
      }
    });
  } else if ( name = request.url.match(/^\/name\/(.+?)\/$/) ) {
    console.log(name[1]);
    response.writeHead(200, {"Content-Type": "text/html"});
    response.end('["a", "b", "c"]');
  } else {
    response.writeHead(302, {
      'Location': 'http://localhost:5000'
    });
    response.end();
  }
});

const port = process.env.PORT || 5000;
server.listen(port, function() {
  console.log("To view your app, open this link in your browser: http://localhost:" + port);
});
