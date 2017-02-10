#!/usr/bin/env node
/* jshint node:true, esversion:6 */
'use strict';

/**
 * This is a quick script I wrote to change C octal escapes (`\002`)
 * into C and Rust compatable hex escapes (`\x02`)
 * Pipe into stdin to use.
 * Might break on Windows. Don't care.
 */

var input = require('fs').readFileSync('/dev/stdin', 'utf8');
console.log(input.replace(/\\([0-9]{3})/g, function(_, octal) {
  var hex = parseInt(octal, 8).toString(16);
  if (hex.length == 1) {
    //lazy padding method
    hex = '0'+hex;
  }
  // uppercase isn't required but it looks nicer.
  return '\\x' + hex.toUpperCase();
}));
