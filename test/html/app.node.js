const http = require('http');
const html_text = `
<!DOCTYPE html>
<html>
<head>
    <title>Node App</title>
    <link rel="stylesheet" href="//unpkg.com/bootstrap/dist/css/bootstrap.min.css">
</head>
<body class="p-5 text-center">
    <p><img src="//images.unsplash.com/photo-1465153690352-10c1b29577f8?fit=crop&w=200&h=200"
            class="img-fluid rounded-circle"></p>
    <h1 class="mb-3">Hello, world!</h1>
    <p>Serving from Node version ${process.version}</p>
</body>
</html>
`
const server = http.createServer((req, res) => {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'text/html');
    res.end(html_text);
});
server.listen(process.env.PORT || 8080);
