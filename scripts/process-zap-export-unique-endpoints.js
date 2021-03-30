const fs = require('fs');
const { Parser } = require('json2csv');

const endpointFile = process.argv[2];
if (!endpointFile) {
  console.log('Usage: node ... [endpoint-file]');
  process.exit(1);
}

const json = JSON.parse(fs.readFileSync(endpointFile).toString());

const uniqueEndpoints = {};

json.forEach(x => {
  uniqueEndpoints[x.uriPlain] = true;
});

console.log(Object.keys(uniqueEndpoints).join('\n'));
