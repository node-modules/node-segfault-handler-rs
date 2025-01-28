const { register, causeSigsegv } = require('../../index.js');

register();

setTimeout(() => {
  causeSigsegv();
}, 100);