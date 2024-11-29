function check() {
  const data = JSON.parse(Host.inputString());
  Host.outputString(JSON.stringify(data));
}

module.exports = { check };
