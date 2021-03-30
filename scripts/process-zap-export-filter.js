const fs = require('fs');
const { Parser } = require('json2csv');

const endpointFile = process.argv[2];
const configFile = process.argv[3];
if (!endpointFile || !configFile) {
  console.log('Usage: node ... [endpoint-file] [config-file]');
  process.exit(1);
}

const endpoints = JSON.parse(fs.readFileSync(endpointFile).toString());
const config = JSON.parse(fs.readFileSync(configFile).toString());

function shouldExcludeByUri(endpoint)
{
  let matches = false;
  config.excludeUri.forEach(regexStr => {
    matches = !!endpoint.uriPlain.match(regexStr) || matches;
  });
  return matches;
}

function shouldExcludeByHost(endpoint)
{
  let matches = false;
  config.excludeHost.forEach(regexStr => {
    matches = !!endpoint.host.match(regexStr) || matches;
  });
  return matches;
}

const filteredEndpoints = [];
endpoints.forEach(x => {
  if (!shouldExcludeByHost(x) && !shouldExcludeByUri(x))
  {
    filteredEndpoints.push(x);
  }
});

console.error(`filtered out ${endpoints.length - filteredEndpoints.length}/${endpoints.length} endpoints`);

console.log(JSON.stringify(filteredEndpoints));
