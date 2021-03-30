const fs = require('fs');
const { Parser } = require('json2csv');

const endpointFile = process.argv[2];
if (!endpointFile) {
  console.log('Usage: node ... [endpoint-file]');
  process.exit(1);
}

const json = JSON.parse(fs.readFileSync(endpointFile).toString());

json.forEach(x => {
  delete x.reqHeader;
  // delete x.reqBody;
  delete x.resHeader;
  // delete x.resBody;

  const url = new URL(x.uri);
  x.host = url.host;
  x.protocol = url.protocol;
  x.path = url.pathname;

  if (Array.isArray(x.queryParams)) {
    x.queryParams = x.queryParams.join('&');
  }

  if (x.reqBody.length > 10000) {
    x.reqBody = 'too large for gsheets';
  }

  if (x.resBody.length > 10000) {
    x.resBody = 'too large for gsheets';
  }
});

const parser = new Parser({
  header: false,
  delimiter: ';',
  fields: ['id', 'protocol', 'host', 'method', 'path', 'queryParams', 'reqBody', 'resBody']
});
console.log(parser.parse(json));
