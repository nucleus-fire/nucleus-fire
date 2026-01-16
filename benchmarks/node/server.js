const express = require('express');
const app = express();
const port = 3001;

app.get('/', (req, res) => {
  res.send('<!DOCTYPE html><html><body><h1>Hello World</h1></body></html>');
});

app.listen(port, () => {
  console.log(`Node app listening on port ${port}`);
});
